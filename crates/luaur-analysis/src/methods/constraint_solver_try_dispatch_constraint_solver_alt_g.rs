use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_table_type::get_mutable_table_type;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_table_type::get_table_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::occurs_check_type_utils::occurs_check_type_id_type_id;
use crate::functions::saturate_arguments::saturate_arguments;
use crate::functions::shallow_clone_clone_alt_b::shallow_clone;
use crate::records::clone_state::CloneState;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::infinite_type_finder::InfiniteTypeFinder;
use crate::records::instantiation_queuer::InstantiationQueuer;
use crate::records::instantiation_queuer_deprecated::InstantiationQueuerDeprecated;
use crate::records::instantiation_signature::InstantiationSignature;
use crate::records::iterative_type_visitor::IterativeTypeVisitorTrait;
use crate::records::metatable_type::MetatableType;
use crate::records::occurs_check_failed::OccursCheckFailed;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::table_type::TableType;
use crate::records::type_alias_expansion_constraint::TypeAliasExpansionConstraint;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::unknown_symbol::{Context, UnknownSymbol};
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::type_error_data::TypeErrorData;
use luaur_ast::records::ast_name::AstName;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn try_dispatch_type_alias_expansion_constraint_not_null_constraint(
        &mut self,
        c: &TypeAliasExpansionConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let petv = unsafe { get_type_id::<PendingExpansionType>(follow_type_id(c.target)) };
        if petv.is_null() {
            self.unblock_type_id_location(c.target, unsafe { (*constraint).location });
            return true;
        }

        let petv = unsafe { &*petv };
        let alias_name = ast_name_to_string(petv.name);
        let alias_prefix = petv.prefix.map(ast_name_to_string);
        let raw_type_arguments = petv.type_arguments.clone();
        let raw_pack_arguments = petv.pack_arguments.clone();

        let tf = unsafe {
            if let Some(prefix) = &alias_prefix {
                (*(*constraint).scope).lookup_imported_type(prefix, &alias_name)
            } else {
                (*(*constraint).scope).lookup_type(&alias_name)
            }
        };

        let Some(tf) = tf else {
            self.report_error_type_error_data_location(
                TypeErrorData::UnknownSymbol(UnknownSymbol::new(alias_name, Context::Type)),
                unsafe { &(*constraint).location },
            );
            bind_alias_expansion_result(self, c, constraint, unsafe {
                (*self.builtin_types).errorType
            });
            return true;
        };

        if unsafe {
            !get_type_id::<TypeFunctionInstanceType>(follow_type_id(tf.r#type())).is_null()
        } {
            self.push_constraint(
                unsafe { core::ptr::NonNull::new_unchecked((*constraint).scope) },
                unsafe { (*constraint).location },
                ConstraintV::Reduce(ReduceConstraint { ty: tf.r#type() }),
            );
        }

        let lhs = unsafe { follow_type_id(c.target) };
        let rhs = tf.r#type();
        if occurs_check_type_id_type_id(lhs, rhs) {
            self.report_error_type_error_data_location(
                TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
                unsafe { &(*constraint).location },
            );
            bind_alias_expansion_result(self, c, constraint, unsafe {
                (*self.builtin_types).errorType
            });
            return true;
        }

        if tf.type_params().is_empty() && tf.type_pack_params().is_empty() {
            bind_alias_expansion_result(self, c, constraint, tf.r#type());
            return true;
        }

        let (type_arguments, pack_arguments) = unsafe {
            saturate_arguments(
                &mut *self.arena,
                &mut *self.builtin_types,
                &tf,
                &raw_type_arguments,
                &raw_pack_arguments,
            )
        };

        let same_types = type_arguments.len() == tf.type_params().len()
            && type_arguments
                .iter()
                .zip(tf.type_params())
                .all(|(arg, param)| *arg == param.ty);
        let same_packs = pack_arguments.len() == tf.type_pack_params().len()
            && pack_arguments
                .iter()
                .zip(tf.type_pack_params())
                .all(|(arg, param)| *arg == param.tp);

        if same_types && same_packs {
            bind_alias_expansion_result(self, c, constraint, tf.r#type());
            return true;
        }

        let signature = InstantiationSignature {
            fn_sig: tf.clone(),
            arguments: type_arguments.clone(),
            pack_arguments: pack_arguments.clone(),
        };

        if let Some(cached) = self.instantiated_aliases.find(&signature).copied() {
            bind_alias_expansion_result(self, c, constraint, cached);
            return true;
        }

        let mut itf = InfiniteTypeFinder::infinite_type_finder_infinite_type_finder(
            self,
            &signature,
            unsafe { core::ptr::NonNull::new_unchecked((*constraint).scope) },
        );
        itf.run_type_id(tf.r#type());

        if itf.found_infinite_type {
            bind_alias_expansion_result(self, c, constraint, unsafe {
                (*self.builtin_types).errorType
            });
            unsafe {
                (*(*constraint).scope)
                    .invalid_type_aliases
                    .try_insert(alias_name.clone(), (*constraint).location);
            }
            return true;
        }

        let mut apply_type_function =
            crate::records::apply_type_function::ApplyTypeFunction::apply_type_function(self.arena);
        for (i, ty) in type_arguments.iter().enumerate() {
            *apply_type_function
                .type_arguments
                .get_or_insert(tf.type_params()[i].ty) = *ty;
        }

        for (i, tp) in pack_arguments.iter().enumerate() {
            *apply_type_function
                .type_pack_arguments
                .get_or_insert(tf.type_pack_params()[i].tp) = *tp;
        }

        let Some(mut instantiated) = apply_type_function.substitute_type_id(tf.r#type()) else {
            bind_alias_expansion_result(self, c, constraint, unsafe {
                (*self.builtin_types).errorType
            });
            return true;
        };

        let mut target = unsafe { follow_type_id(instantiated) };

        if FFlag::LuauIterativeInstantiationQueuer.get() {
            let mut queuer = InstantiationQueuer::instantiation_queuer(
                unsafe { core::ptr::NonNull::new_unchecked((*constraint).scope) },
                unsafe { &(*constraint).location },
                self as *mut ConstraintSolver,
            );
            queuer.run_type_id(target);
        } else {
            let mut queuer = InstantiationQueuerDeprecated::instantiation_queuer_deprecated_instantiation_queuer_deprecated(
                unsafe { core::ptr::NonNull::new_unchecked((*constraint).scope) },
                unsafe { &(*constraint).location },
                self as *mut ConstraintSolver,
            );
            queuer.traverse_type_id(target);
        }

        if unsafe { (*target).persistent || (*target).owning_arena != self.arena } {
            bind_alias_expansion_result(self, c, constraint, target);
            return true;
        }

        let tf_table = get_table_type(tf.r#type())
            .map(|table| table as *const TableType)
            .unwrap_or(core::ptr::null());
        let target_table = get_table_type(target)
            .map(|table| table as *const TableType)
            .unwrap_or(core::ptr::null());
        let needs_clone = unsafe { follow_type_id(tf.r#type()) == target }
            || (!tf_table.is_null() && tf_table == target_table)
            || type_arguments.iter().any(|other| *other == target);

        let mut table = get_mutable_table_type(target);
        if !table.is_null() {
            if needs_clone {
                if unsafe { !get_type_id::<MetatableType>(target).is_null() } {
                    let mut clone_state = unsafe { CloneState::new(&mut *self.builtin_types) };
                    instantiated =
                        unsafe { shallow_clone(target, &mut *self.arena, &mut clone_state, true) };
                    let metatable = unsafe { get_mutable_type_id::<MetatableType>(instantiated) };
                    unsafe {
                        (*metatable).table = shallow_clone(
                            (*metatable).table(),
                            &mut *self.arena,
                            &mut clone_state,
                            true,
                        );
                        table = get_mutable_type_id::<TableType>((*metatable).table());
                    }
                } else if unsafe { !get_type_id::<TableType>(target).is_null() } {
                    let mut clone_state = unsafe { CloneState::new(&mut *self.builtin_types) };
                    instantiated =
                        unsafe { shallow_clone(target, &mut *self.arena, &mut clone_state, true) };
                    table = unsafe { get_mutable_type_id::<TableType>(instantiated) };
                }

                target = unsafe { follow_type_id(instantiated) };
            }

            unsafe {
                (*table).instantiated_type_params = type_arguments.clone();
                (*table).instantiated_type_pack_params = pack_arguments.clone();
                (*table).definition_location = (*constraint).location;
                if let Some(module) = &self.module {
                    (*table).definition_module_name = module.name.clone();
                }
            }
        }

        bind_alias_expansion_result(self, c, constraint, target);
        self.instantiated_aliases.try_insert(signature, target);

        true
    }
}

fn bind_alias_expansion_result(
    solver: &mut ConstraintSolver,
    c: &TypeAliasExpansionConstraint,
    constraint: *const Constraint,
    result: crate::type_aliases::type_id::TypeId,
) {
    let c_target = unsafe { follow_type_id(c.target) };

    if occurs_check_type_id_type_id(c_target, result) {
        solver.report_error_type_error_data_location(
            TypeErrorData::OccursCheckFailed(OccursCheckFailed::default()),
            unsafe { &(*constraint).location },
        );
        solver.bind_not_null_constraint_type_id_type_id(constraint, c_target, unsafe {
            (*solver.builtin_types).errorType
        });
    } else {
        solver.bind_not_null_constraint_type_id_type_id(constraint, c_target, result);
    }
}

fn ast_name_to_string(name: AstName) -> String {
    if name.value.is_null() {
        String::new()
    } else {
        unsafe { core::ffi::CStr::from_ptr(name.value) }
            .to_string_lossy()
            .into_owned()
    }
}
