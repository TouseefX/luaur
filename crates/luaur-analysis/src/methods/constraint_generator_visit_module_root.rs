use crate::enums::control_flow::ControlFlow;
use crate::enums::polarity::Polarity;
use crate::functions::add_all_as_dependencies::add_all_as_dependencies;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::checkpoint::checkpoint;
use crate::functions::follow_type::follow_type_id;
use crate::functions::for_each_constraint::for_each_constraint;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::function_type::FunctionType;
use crate::records::generalization_constraint::GeneralizationConstraint;
use crate::records::interior_free_types::InteriorFreeTypes;
use crate::records::module::Module;
use crate::records::pack_subtype_constraint::PackSubtypeConstraint;
use crate::records::scope::Scope;
use crate::records::simplify_constraint::SimplifyConstraint;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use alloc::rc::Rc;
use alloc::sync::Arc;
use alloc::vec::Vec;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::macros::luau_timetrace_scope::LUAU_TIMETRACE_SCOPE;
use luaur_common::FFlag;

impl ConstraintGenerator {
    pub fn visit_module_root(&mut self, block: *mut AstStatBlock) {
        LUAU_TIMETRACE_SCOPE!("ConstraintGenerator::visitModuleRoot", "Typechecking");

        LUAU_ASSERT!(self.scopes.is_empty());
        LUAU_ASSERT!(self.root_scope.is_null());

        let scope: ScopePtr = Arc::new(Scope::new(self.global_scope.as_ref().unwrap(), 0));
        self.root_scope = scope.as_ref() as *const Scope as *mut Scope;
        self.scopes
            .push((unsafe { (*block).base.base.location }, scope.clone()));
        unsafe {
            (*self.root_scope).location = (*block).base.base.location;
        }
        if let Some(module) = &self.module {
            let module_ptr = Arc::as_ptr(module) as *mut Module;
            unsafe {
                *(*module_ptr)
                    .ast_scopes
                    .get_or_insert(block as *const AstNode) =
                    scope.as_ref() as *const Scope as *mut Scope;
            }
        }

        self.interior_free_types.push(InteriorFreeTypes::default());

        let local_type_function_scope: ScopePtr =
            Arc::new(Scope::new(self.type_function_scope.as_ref().unwrap(), 0));
        unsafe {
            let lhs = local_type_function_scope.as_ref() as *const Scope as *mut Scope;
            (*lhs).location = (*block).base.base.location;
        }
        unsafe {
            (*self.type_function_runtime).root_scope = local_type_function_scope;
        }

        let return_type = self.fresh_type_pack(&scope, Polarity::Positive);
        unsafe {
            (*self.root_scope).return_type = return_type;
        }
        let module_fn_ty = unsafe {
            (*self.arena).add_type(FunctionType::function_type_new(
                (*self.builtin_types).anyTypePack,
                return_type,
                None,
                false,
            ))
        };

        self.prepopulate_global_scope(&scope, block);

        let start = checkpoint(self);

        let cf = self.visit_block_without_child_scope(self.root_scope, block);
        if cf == ControlFlow::None {
            let empty_type_pack = unsafe { (*self.builtin_types).emptyTypePack };
            self.add_constraint_scope_ptr_location_constraint_v(
                &scope,
                unsafe { (*block).base.base.location },
                ConstraintV::PackSubtype(PackSubtypeConstraint {
                    sub_pack: empty_type_pack,
                    super_pack: return_type,
                    returns: false,
                }),
            );
        }

        let end = checkpoint(self);

        let result = unsafe { (*self.arena).add_type(BlockedType::default()) };
        let gen_constraint = self.add_constraint_scope_ptr_location_constraint_v(
            &scope,
            unsafe { (*block).base.base.location },
            ConstraintV::Generalization(GeneralizationConstraint {
                generalized_type: result,
                source_type: module_fn_ty,
                interior_types: Vec::new(),
                has_deprecated_attribute: false,
                deprecated_info: Default::default(),
                no_generics: true,
            }),
        );

        unsafe {
            (*self.root_scope).interior_free_types =
                Some(self.interior_free_types.last().unwrap().types.clone());
            (*self.root_scope).interior_free_type_packs =
                Some(self.interior_free_types.last().unwrap().type_packs.clone());
        }

        unsafe {
            let blocked = getMutable::<BlockedType>(result);
            (*blocked).set_owner(gen_constraint);
        }

        if FFlag::LuauConstraintGraph.get() {
            add_all_as_dependencies(start, end, self, gen_constraint);
        } else {
            for_each_constraint(start, end, self, |c: *mut Constraint| unsafe {
                (*gen_constraint).deprecated_dependencies.push(c);
            });
        }

        self.interior_free_types.pop();

        self.fill_in_inferred_bindings(&scope, block);

        if !self.logger.is_null() {
            unsafe {
                (*self.logger).capture_generation_module(self.module.clone().unwrap());
            }
        }

        let local_types_pairs: Vec<(TypeId, Vec<TypeId>)> = self
            .local_types
            .iter()
            .map(|(ty, domain)| (*ty, domain.order.clone()))
            .collect();
        for (ty, domain) in local_types_pairs {
            // FIXME: This isn't the most efficient thing.
            let mut domain_ty = unsafe { (*self.builtin_types).neverType };
            for d in domain {
                let d_followed = unsafe { follow_type_id(d) };
                if d_followed == ty {
                    continue;
                }
                domain_ty =
                    self.simplify_union(scope.clone(), Location::default(), domain_ty, d_followed);
            }

            LUAU_ASSERT!(!unsafe { get_type_id::<BlockedType>(ty) }.is_null());
            unsafe {
                (*as_mutable_type_id(ty)).ty = TypeVariant::Bound(domain_ty);
            }
        }

        let unions_to_simplify = self.unions_to_simplify.clone();
        for ty in unions_to_simplify {
            self.add_constraint_scope_ptr_location_constraint_v(
                &scope,
                unsafe { (*block).base.base.location },
                ConstraintV::Simplify(SimplifyConstraint { ty }),
            );
        }
    }
}
