use crate::enums::polarity::Polarity;
use crate::enums::table_state::TableState;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::fresh_type::fresh_type;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::track_interior_free_type::track_interior_free_type;
use crate::records::any_type::AnyType;
use crate::records::blocked_type::BlockedType;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::intersection_builder::IntersectionBuilder;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::never_type::NeverType;
use crate::records::set::Set;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;
use crate::records::union_builder::UnionBuilder;
use crate::records::union_type::UnionType;
use crate::type_aliases::error_type::ErrorType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FFlag;

impl ConstraintSolver {
    pub fn constraint_solver_try_dispatch_has_indexer(
        &mut self,
        _recursion_depth: &mut i32,
        constraint: *const Constraint,
        subject_type: TypeId,
        index_type: TypeId,
        mut result_type: TypeId,
        _seen: &mut DenseHashSet<TypeId>,
    ) -> bool {
        let subject_type = unsafe { follow_type_id(subject_type) };
        let index_type = unsafe { follow_type_id(index_type) };

        if _seen.contains(&subject_type) {
            return false;
        }
        _seen.insert(subject_type);

        if unsafe { !get_type_id::<AnyType>(subject_type).is_null() } {
            self.bind_not_null_constraint_type_id_type_id(constraint, result_type, unsafe {
                (*self.builtin_types).anyType
            });
            return true;
        }

        let free_type = unsafe { get_mutable_type_id::<FreeType>(subject_type) };
        if !free_type.is_null() {
            let upper_bound = unsafe { follow_type_id((*free_type).upper_bound) };

            if let Some(table) = unsafe { get_type_id::<TableType>(upper_bound).as_ref() } {
                if let Some(indexer) = &table.indexer {
                    self.constraint_solver_unify(constraint, index_type, indexer.index_type);
                    self.bind_not_null_constraint_type_id_type_id(
                        constraint,
                        result_type,
                        indexer.index_result_type,
                    );
                    return true;
                }
            } else if let Some(metatable) =
                unsafe { get_type_id::<MetatableType>(upper_bound).as_ref() }
            {
                return self.constraint_solver_try_dispatch_has_indexer(
                    _recursion_depth,
                    constraint,
                    metatable.table(),
                    index_type,
                    result_type,
                    _seen,
                );
            }

            let scope = unsafe { (*free_type).scope };
            let free_result = unsafe {
                fresh_type(
                    &mut *self.arena,
                    &*self.builtin_types,
                    scope,
                    Polarity::Mixed,
                )
            };
            track_interior_free_type(scope, free_result);
            self.bind_not_null_constraint_type_id_type_id(constraint, result_type, free_result);
            result_type = free_result;

            let mut table = TableType::table_type_table_state_type_level_scope(
                TableState::Unsealed,
                TypeLevel::default(),
                scope,
            );
            table.indexer = Some(TableIndexer {
                index_type,
                index_result_type: free_result,
                is_read_only: false,
            });

            let upper_bound = unsafe { (*self.arena).add_type(table) };
            let simplified = self.simplify_intersection_not_null_scope_location_type_id_type_id(
                unsafe { (*constraint).scope },
                unsafe { (*constraint).location },
                unsafe { (*free_type).upper_bound },
                upper_bound,
            );

            if unsafe { !get_type_id::<NeverType>(simplified).is_null() } {
                self.bind_not_null_constraint_type_id_type_id(constraint, result_type, unsafe {
                    (*self.builtin_types).errorType
                });
            } else {
                unsafe {
                    (*free_type).upper_bound = simplified;
                }
            }

            return true;
        }

        if let Some(table) = unsafe { get_mutable_type_id::<TableType>(subject_type).as_mut() } {
            if let Some(indexer) = &table.indexer {
                self.constraint_solver_unify(constraint, index_type, indexer.index_type);
                self.bind_not_null_constraint_type_id_type_id(
                    constraint,
                    result_type,
                    indexer.index_result_type,
                );
                return true;
            }

            if table.state == TableState::Unsealed {
                let scope = table.scope;
                let free_result = unsafe {
                    fresh_type(
                        &mut *self.arena,
                        &*self.builtin_types,
                        scope,
                        Polarity::Mixed,
                    )
                };
                track_interior_free_type(scope, free_result);
                self.bind_not_null_constraint_type_id_type_id(constraint, result_type, free_result);
                table.indexer = Some(TableIndexer {
                    index_type,
                    index_result_type: result_type,
                    is_read_only: false,
                });
                return true;
            }
        }

        if let Some(metatable) = unsafe { get_type_id::<MetatableType>(subject_type).as_ref() } {
            return self.constraint_solver_try_dispatch_has_indexer(
                _recursion_depth,
                constraint,
                metatable.table(),
                index_type,
                result_type,
                _seen,
            );
        }

        let mut extern_type = unsafe { get_type_id::<ExternType>(subject_type) as *mut ExternType };
        while !extern_type.is_null() {
            if let Some(indexer) = unsafe { &(*extern_type).indexer } {
                self.constraint_solver_unify(constraint, index_type, indexer.index_type);
                self.bind_not_null_constraint_type_id_type_id(
                    constraint,
                    result_type,
                    indexer.index_result_type,
                );
                return true;
            }

            extern_type = if let Some(parent) = unsafe { (*extern_type).parent } {
                unsafe { get_type_id::<ExternType>(parent) as *mut ExternType }
            } else {
                core::ptr::null_mut()
            };
        }

        if let Some(it) = unsafe { get_type_id::<IntersectionType>(subject_type).as_ref() } {
            // Indexing into an intersection of types is roughly akin to overload
            // selection: for every type in the intersection where it is well typed
            // to index into _that_ type, we construct an intersection of said result
            // types.
            if FFlag::LuauRemoveConstraintSolverEmplace.get() {
                let mut ib =
                    IntersectionBuilder::intersection_builder(self.arena, self.builtin_types);
                let mut success = false;

                let parts: alloc::vec::Vec<TypeId> = it.parts.clone();
                for part in parts {
                    let r = unsafe { (*self.arena).add_type(BlockedType::default()) };
                    unsafe {
                        (*(get_mutable_type_id::<BlockedType>(r))).set_owner(constraint);
                    }

                    let ok = self.constraint_solver_try_dispatch_has_indexer(
                        _recursion_depth,
                        constraint,
                        part,
                        index_type,
                        r,
                        _seen,
                    );
                    // If we've cut a recursive loop short, skip it.
                    if !ok {
                        continue;
                    }

                    let r = unsafe { follow_type_id(r) };
                    if unsafe { get_type_id::<ErrorType>(r).is_null() } {
                        success = true;
                        ib.add(r);
                    }
                }

                // We need to distinguish between the empty case (there
                // were no valid indexable types) and the bottom type (one of the
                // indexable result types was never). UnionBuilder will opt to
                // only record that its seen a top type as an optimization. we
                // add a flag to distinguish these cases.
                if success {
                    let built = ib.build();
                    self.bind_not_null_constraint_type_id_type_id(constraint, result_type, built);
                } else {
                    self.bind_not_null_constraint_type_id_type_id(
                        constraint,
                        result_type,
                        unsafe { (*self.builtin_types).errorType },
                    );
                }
            } else {
                let mut parts: Set<TypeId> = Set::new(core::ptr::null());
                let part_list: alloc::vec::Vec<TypeId> = it.parts.clone();
                for part in part_list {
                    parts.insert(&unsafe { follow_type_id(part) });
                }

                let mut results: Set<TypeId> = Set::new(core::ptr::null());

                let parts_iter: alloc::vec::Vec<TypeId> = parts.iter().copied().collect();
                for part in parts_iter {
                    let r = unsafe { (*self.arena).add_type(BlockedType::default()) };
                    unsafe {
                        (*(get_mutable_type_id::<BlockedType>(r))).set_owner(constraint);
                    }

                    let ok = self.constraint_solver_try_dispatch_has_indexer(
                        _recursion_depth,
                        constraint,
                        part,
                        index_type,
                        r,
                        _seen,
                    );
                    // If we've cut a recursive loop short, skip it.
                    if !ok {
                        continue;
                    }

                    let r = unsafe { follow_type_id(r) };
                    if unsafe { get_type_id::<ErrorType>(r).is_null() } {
                        results.insert(&r);
                    }
                }

                if results.size() == 0 {
                    self.bind_not_null_constraint_type_id_type_id(
                        constraint,
                        result_type,
                        unsafe { (*self.builtin_types).errorType },
                    );
                } else if results.size() == 1 {
                    let first = *results.iter().next().unwrap();
                    self.bind_not_null_constraint_type_id_type_id(constraint, result_type, first);
                } else {
                    let parts_vec: alloc::vec::Vec<TypeId> = results.iter().copied().collect();
                    let mutable_ty = unsafe { as_mutable_type_id(result_type) };
                    unsafe {
                        (*mutable_ty).ty =
                            TypeVariant::Intersection(IntersectionType { parts: parts_vec });
                    }
                    let location = unsafe { (*constraint).location };
                    self.unblock_type_id_location(result_type, location);
                }
            }

            return true;
        }

        if let Some(ut) = unsafe { get_type_id::<UnionType>(subject_type).as_ref() } {
            // Indexing into a union of types means constructing a union of
            // results: we don't know _which_ type it could be.
            if FFlag::LuauRemoveConstraintSolverEmplace.get() {
                let mut ub = UnionBuilder::union_builder(self.arena, self.builtin_types);
                let mut success = false;

                let options: alloc::vec::Vec<TypeId> = ut.options.clone();
                for option in options {
                    let r = unsafe { (*self.arena).add_type(BlockedType::default()) };
                    unsafe {
                        (*(get_mutable_type_id::<BlockedType>(r))).set_owner(constraint);
                    }

                    let ok = self.constraint_solver_try_dispatch_has_indexer(
                        _recursion_depth,
                        constraint,
                        option,
                        index_type,
                        r,
                        _seen,
                    );
                    // If we've cut a recursive loop short, skip it.
                    if !ok {
                        continue;
                    }

                    let r = unsafe { follow_type_id(r) };
                    success = true;
                    ub.add(r);
                }

                // We need to distinguish between the empty case (there
                // were no valid indexable types) and the top type (one of the
                // indexable result types was unknown). UnionBuilder will opt to
                // only record that its seen a top type as an optimization. we
                // add a flag to distinguish these cases.
                if success {
                    let built = ub.build();
                    self.bind_not_null_constraint_type_id_type_id(constraint, result_type, built);
                } else {
                    self.bind_not_null_constraint_type_id_type_id(
                        constraint,
                        result_type,
                        unsafe { (*self.builtin_types).errorType },
                    );
                }
            } else {
                let mut parts: Set<TypeId> = Set::new(core::ptr::null());
                let option_list: alloc::vec::Vec<TypeId> = ut.options.clone();
                for part in option_list {
                    parts.insert(&unsafe { follow_type_id(part) });
                }

                let mut results: Set<TypeId> = Set::new(core::ptr::null());

                let parts_iter: alloc::vec::Vec<TypeId> = parts.iter().copied().collect();
                for part in parts_iter {
                    let r = unsafe { (*self.arena).add_type(BlockedType::default()) };
                    unsafe {
                        (*(get_mutable_type_id::<BlockedType>(r))).set_owner(constraint);
                    }

                    let ok = self.constraint_solver_try_dispatch_has_indexer(
                        _recursion_depth,
                        constraint,
                        part,
                        index_type,
                        r,
                        _seen,
                    );
                    // If we've cut a recursive loop short, skip it.
                    if !ok {
                        continue;
                    }

                    let r = unsafe { follow_type_id(r) };
                    results.insert(&r);
                }

                if results.size() == 0 {
                    self.bind_not_null_constraint_type_id_type_id(
                        constraint,
                        result_type,
                        unsafe { (*self.builtin_types).errorType },
                    );
                } else if results.size() == 1 {
                    let first_result = *results.iter().next().unwrap();
                    if !FFlag::LuauConstraintGraph.get() {
                        // bind will already shift references.
                        self.deprecate_d_shift_references(result_type, first_result);
                    }
                    self.bind_not_null_constraint_type_id_type_id(
                        constraint,
                        result_type,
                        first_result,
                    );
                } else {
                    let options_vec: alloc::vec::Vec<TypeId> = results.iter().copied().collect();
                    let mutable_ty = unsafe { as_mutable_type_id(result_type) };
                    unsafe {
                        (*mutable_ty).ty = TypeVariant::Union(UnionType {
                            options: options_vec,
                        });
                    }
                    let location = unsafe { (*constraint).location };
                    self.unblock_type_id_location(result_type, location);
                }
            }

            return true;
        }

        self.bind_not_null_constraint_type_id_type_id(constraint, result_type, unsafe {
            (*self.builtin_types).errorType
        });
        true
    }
}
