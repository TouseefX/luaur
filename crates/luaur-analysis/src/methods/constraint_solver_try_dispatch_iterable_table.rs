use crate::enums::polarity::Polarity;
use crate::enums::table_state::TableState;
use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::follow_type::follow_type_id;
use crate::functions::fresh_type::fresh_type;
use crate::functions::get_mutable_type::getMutable;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::instantiate::instantiate;
use crate::functions::track_interior_free_type::track_interior_free_type;
use crate::records::any_type::AnyType;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::iterable_constraint::IterableConstraint;
use crate::records::metatable_type::MetatableType;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::reduce_constraint::ReduceConstraint;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_level::TypeLevel;
use crate::records::unification_too_complex::UnificationTooComplex;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::props_type::Props;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn try_dispatch_iterable_table(
        &mut self,
        iterator_ty: TypeId,
        c: &IterableConstraint,
        constraint: *const Constraint,
        force: bool,
    ) -> bool {
        let iterator_ty = unsafe { follow_type_id(iterator_ty) };

        if unsafe { !get_type_id::<FreeType>(iterator_ty).is_null() } {
            let scope = unsafe { (*constraint).scope };
            let key_ty = unsafe {
                fresh_type(
                    &mut *self.arena,
                    &*self.builtin_types,
                    scope,
                    Polarity::Mixed,
                )
            };
            let value_ty = unsafe {
                fresh_type(
                    &mut *self.arena,
                    &*self.builtin_types,
                    scope,
                    Polarity::Mixed,
                )
            };
            track_interior_free_type(scope, key_ty);
            track_interior_free_type(scope, value_ty);

            let props = Props::default();
            let table_ty = unsafe {
                (*self.arena).add_type(
                    TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                        &props,
                        Some(TableIndexer {
                            index_type: key_ty,
                            index_result_type: value_ty,
                            is_read_only: false,
                        }),
                        TypeLevel::default(),
                        scope,
                        TableState::Sealed,
                    ),
                )
            };

            self.constraint_solver_unify(constraint, iterator_ty, table_ty);

            let mut it = c.variables.iter();
            if let Some(ty) = it.next() {
                self.bind_not_null_constraint_type_id_type_id(constraint, *ty, key_ty);
            }
            if let Some(ty) = it.next() {
                self.bind_not_null_constraint_type_id_type_id(constraint, *ty, value_ty);
            }

            return true;
        }

        if unsafe { !get_type_id::<AnyType>(iterator_ty).is_null() } {
            self.unpack_iterable_variables(constraint, c, unsafe { (*self.builtin_types).anyType });
            return true;
        }

        if unsafe { !get_type_id::<NeverType>(iterator_ty).is_null() } {
            self.unpack_iterable_variables(constraint, c, unsafe {
                (*self.builtin_types).neverType
            });
            return true;
        }

        // Irksome: I don't think we have any way to guarantee that this table
        // type never has a metatable.

        if let Some(iterator_table) = unsafe { get_type_id::<TableType>(iterator_ty).as_ref() } {
            if iterator_table.state == TableState::Free && !force {
                return self.block_type_id_not_null_constraint(iterator_ty, constraint);
            }

            if let Some(indexer) = iterator_table.indexer {
                let value_type = if FFlag::LuauRefineNilFromTableIndexerResultType.get() {
                    let intersection_with_not_nil = unsafe {
                        (*self.arena).add_type_function_type_function_initializer_list_type_id(
                            &(*self.builtin_types).typeFunctions.intersect_func,
                            &[indexer.index_result_type, (*self.builtin_types).notNilType],
                        )
                    };

                    unsafe {
                        self.push_constraint(
                            NonNull::new((*constraint).scope).unwrap(),
                            (*constraint).location,
                            ConstraintV::Reduce(ReduceConstraint {
                                ty: intersection_with_not_nil,
                            }),
                        );
                    }

                    intersection_with_not_nil
                } else {
                    indexer.index_result_type
                };

                let mut expected_variables = alloc::vec![indexer.index_type, value_type];
                while expected_variables.len() < c.variables.len() {
                    expected_variables.push(unsafe { (*self.builtin_types).errorType });
                }

                for (variable, expected) in c.variables.iter().zip(expected_variables.iter()) {
                    self.constraint_solver_unify(constraint, *variable, *expected);
                    self.bind_not_null_constraint_type_id_type_id(constraint, *variable, *expected);
                }
            } else {
                self.unpack_iterable_variables(constraint, c, unsafe {
                    (*self.builtin_types).errorType
                });
            }

            return true;
        }

        // else if (std::optional<TypeId> iterFn = findMetatableEntry(builtinTypes, errors, iteratorTy, "__iter", Location{}))
        let iter_fn = find_metatable_entry(
            self.builtin_types,
            &mut self.errors,
            iterator_ty,
            "__iter",
            Location::default(),
        );
        if let Some(iter_fn) = iter_fn {
            if self.is_blocked_type_id(iter_fn) {
                return self.block_type_id_not_null_constraint(iter_fn, constraint);
            }

            let scope = unsafe { (*constraint).scope };
            let instantiated_iter_fn = instantiate(
                self.builtin_types,
                self.arena,
                &mut self.limits as *mut TypeCheckLimits,
                scope,
                iter_fn,
            );

            if let Some(instantiated_iter_fn) = instantiated_iter_fn {
                let iter_ftv = unsafe { get_type_id::<FunctionType>(instantiated_iter_fn) };
                if !iter_ftv.is_null() {
                    let iter_ftv = unsafe { &*iter_ftv };

                    let expected_iter_args = unsafe {
                        (*self.arena).add_type_pack_initializer_list_type_id(&[iterator_ty])
                    };
                    self.constraint_solver_unify(
                        constraint,
                        iter_ftv.arg_types,
                        expected_iter_args,
                    );

                    let iter_rets = extend_type_pack(
                        unsafe { &mut *self.arena },
                        self.builtin_types,
                        iter_ftv.ret_types,
                        2,
                        alloc::vec::Vec::new(),
                    );

                    if iter_rets.head.len() < 1 {
                        // We've done what we can; this will get reported as an
                        // error by the type checker.
                        return true;
                    }

                    let next_fn_ty = iter_rets.head[0];

                    let instantiated_next_fn = instantiate(
                        self.builtin_types,
                        self.arena,
                        &mut self.limits as *mut TypeCheckLimits,
                        scope,
                        next_fn_ty,
                    );

                    if let Some(instantiated_next_fn) = instantiated_next_fn {
                        let next_fn = unsafe { get_type_id::<FunctionType>(instantiated_next_fn) };

                        // If nextFn is nullptr, then the iterator function has an improper signature.
                        if !next_fn.is_null() {
                            let ret_types = unsafe { (*next_fn).ret_types };
                            self.unpack_and_assign(
                                c.variables.clone(),
                                ret_types,
                                NonNull::new(constraint as *mut Constraint).unwrap(),
                            );
                        }

                        return true;
                    } else {
                        let location = unsafe { (*constraint).location };
                        self.report_error_type_error_data_location(
                            TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
                            &location,
                        );
                    }
                } else {
                    // TODO: Support __call and function overloads (what does an overload even mean for this?)
                }
            } else {
                let location = unsafe { (*constraint).location };
                self.report_error_type_error_data_location(
                    TypeErrorData::UnificationTooComplex(UnificationTooComplex::default()),
                    &location,
                );
            }

            return true;
        }

        // else if (auto iteratorMetatable = get<MetatableType>(iteratorTy))
        if let Some(iterator_metatable) =
            unsafe { get_type_id::<MetatableType>(iterator_ty).as_ref() }
        {
            // If the metatable does not contain a `__iter` metamethod, then we iterate over the table part of the metatable.
            return self.try_dispatch_iterable_table(iterator_metatable.table, c, constraint, force);
        }

        if let Some(primitive_ty) = unsafe { get_type_id::<PrimitiveType>(iterator_ty).as_ref() } {
            if primitive_ty.r#type == PrimitiveType::Table {
                self.unpack_iterable_variables(constraint, c, unsafe {
                    (*self.builtin_types).unknownType
                });
                return true;
            }
        }

        self.unpack_iterable_variables(constraint, c, unsafe { (*self.builtin_types).errorType });

        true
    }

    fn unpack_iterable_variables(
        &mut self,
        constraint: *const Constraint,
        c: &IterableConstraint,
        ty: TypeId,
    ) {
        for var_ty in &c.variables {
            self.bind_not_null_constraint_type_id_type_id(constraint, *var_ty, ty);
        }
    }
}
