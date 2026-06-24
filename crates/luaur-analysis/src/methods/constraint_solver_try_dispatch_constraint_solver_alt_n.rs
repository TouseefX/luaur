use crate::enums::table_state::TableState;
use crate::records::assign_index_constraint::AssignIndexConstraint;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::intersection_type::IntersectionType;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type as TableTypeRec;
use crate::records::table_type::TableType;
use crate::records::type_ids::TypeIds;
use crate::records::type_level::TypeLevel;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

use crate::functions::add_union::add_union;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_utils::follow_optional_ty;
use crate::functions::simplify_intersection_simplify::simplify_intersection;

use crate::records::builtin_types::BuiltinTypes;

use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintSolver {
    pub fn try_dispatch_assign_index_constraint_not_null_constraint(
        &mut self,
        c: &AssignIndexConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let lhs_type: TypeId = unsafe { follow_type_id(c.lhs_type) };
        let index_type: TypeId = unsafe { follow_type_id(c.index_type) };
        let rhs_type: TypeId = unsafe { follow_type_id(c.rhs_type) };

        if self.is_blocked_type_id(lhs_type) {
            return self.block_type_id_not_null_constraint(lhs_type, constraint);
        }

        // Important: In every codepath through this function, the type `c.propType`
        // must be bound to something.
        let mut table_stuff = |lhs_table: &mut TableType| -> Option<bool> {
            if lhs_table.indexer.is_some() {
                let indexer: &TableIndexer = lhs_table.indexer.as_ref().unwrap();

                self.constraint_solver_unify(constraint, index_type, indexer.index_type);
                self.constraint_solver_unify(constraint, rhs_type, indexer.index_result_type);

                let prop_bound = self.bind_not_null_constraint_type_id_type_id(
                    constraint,
                    c.prop_type,
                    add_union(
                        self.arena,
                        self.builtin_types,
                        &[indexer.index_result_type, unsafe {
                            (*self.builtin_types).nilType
                        }],
                    ),
                );

                // Note: bind_not_null_constraint_* return type is () in Rust bindings,
                // but we still preserve the original logic shape.
                let _ = prop_bound;

                return Some(true);
            }

            if lhs_table.state == TableState::Unsealed || lhs_table.state == TableState::Free {
                lhs_table.indexer = Some(TableIndexer {
                    index_type,
                    index_result_type: rhs_type,
                    is_read_only: false,
                });

                self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, rhs_type);
                return Some(true);
            }

            None
        };

        let lhs_free: *mut FreeType = unsafe {
            crate::functions::get_mutable_type::get_mutable_type_id::<FreeType>(lhs_type)
        };
        if !lhs_free.is_null() {
            let lhs_upper = unsafe { follow_type_id((*lhs_free).upper_bound) };
            let lhs_table: *mut TableType = unsafe {
                crate::functions::get_mutable_type::get_mutable_type_id::<TableType>(lhs_upper)
            };
            if !lhs_table.is_null() {
                let res = unsafe { table_stuff(&mut *lhs_table) };
                if let Some(v) = res {
                    return v;
                }
            }

            let new_upper_bound = unsafe {
                (*self.arena).add_type(
                    TableType::table_type_props_optional_table_indexer_type_level_scope_table_state(
                        &Default::default(),
                        Some(TableIndexer {
                            index_type,
                            index_result_type: rhs_type,
                            is_read_only: false,
                        }),
                        TypeLevel::default(),
                        (*constraint).scope,
                        TableState::Free,
                    ),
                )
            };

            let new_table: *const TableType = unsafe {
                crate::functions::get_type_alt_j::get_type_id::<TableType>(new_upper_bound)
            };
            LUAU_ASSERT!(!new_table.is_null());

            self.constraint_solver_unify(constraint, lhs_type, new_upper_bound);

            let new_table_mut: *mut TableType = unsafe {
                crate::functions::get_mutable_type::get_mutable_type_id::<TableType>(
                    new_upper_bound,
                )
            };
            LUAU_ASSERT!(!new_table_mut.is_null());
            LUAU_ASSERT!(unsafe { (*new_table_mut).indexer.is_some() });

            let idx_res = unsafe { (*new_table_mut).indexer.as_ref().unwrap().index_result_type };
            self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, idx_res);
            return true;
        }

        let lhs_table: *mut TableType = unsafe {
            crate::functions::get_mutable_type::get_mutable_type_id::<TableType>(lhs_type)
        };
        if !lhs_table.is_null() {
            let res = unsafe { table_stuff(&mut *lhs_table) };
            if let Some(v) = res {
                return v;
            }
        }

        let mut lhs_extern_type: *mut ExternType = unsafe {
            crate::functions::get_type_alt_j::get_type_id::<ExternType>(lhs_type) as *mut ExternType
        };
        if !lhs_extern_type.is_null() {
            loop {
                if unsafe { (*lhs_extern_type).indexer.is_some() } {
                    let indexer = unsafe { (*lhs_extern_type).indexer.as_ref().unwrap() };
                    self.constraint_solver_unify(constraint, index_type, indexer.index_type);
                    self.constraint_solver_unify(constraint, rhs_type, indexer.index_result_type);

                    let res_ty = add_union(
                        self.arena,
                        self.builtin_types,
                        &[indexer.index_result_type, unsafe {
                            (*self.builtin_types).nilType
                        }],
                    );
                    self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, res_ty);
                    return true;
                }

                if let Some(parent) = unsafe { (*lhs_extern_type).parent } {
                    lhs_extern_type = unsafe {
                        crate::functions::get_type_alt_j::get_type_id::<ExternType>(parent)
                            as *mut ExternType
                    };
                    continue;
                }

                break;
            }
            return true;
        }

        let lhs_intersection: *mut IntersectionType = unsafe {
            crate::functions::get_mutable_type::get_mutable_type_id::<IntersectionType>(lhs_type)
        };
        if !lhs_intersection.is_null() {
            let mut parts = TypeIds::type_ids();

            // The port should iterate `lhsIntersection` similarly to C++ range-for.
            // Use `begin_mut/end` style only if available; otherwise, fall back to `parts`.
            // Here we call into the existing iterator API on IntersectionType if present.
            // If IntersectionType has a public `parts` field, we can use it directly.
            let intersection_parts: &alloc::vec::Vec<TypeId> =
                unsafe { &(*lhs_intersection).parts };
            for &t in intersection_parts.iter() {
                let followed = unsafe { follow_type_id(t) };

                let tbl_ptr: *mut TableType = unsafe {
                    crate::functions::get_mutable_type::get_mutable_type_id::<TableType>(followed)
                };
                if !tbl_ptr.is_null() {
                    if let Some(indexer) = unsafe { &(*tbl_ptr).indexer } {
                        self.constraint_solver_unify(constraint, index_type, indexer.index_type);
                        parts.insert_type_id(indexer.index_result_type);
                    }

                    if unsafe {
                        (*tbl_ptr).state == TableState::Unsealed
                            || (*tbl_ptr).state == TableState::Free
                    } {
                        unsafe {
                            (*tbl_ptr).indexer = Some(TableIndexer {
                                index_type,
                                index_result_type: rhs_type,
                                is_read_only: false,
                            });
                        }
                        parts.insert_type_id(rhs_type);
                    }

                    continue;
                }

                let cls_ptr: *const ExternType = unsafe {
                    crate::functions::get_type_alt_j::get_type_id::<ExternType>(followed)
                };
                if !cls_ptr.is_null() {
                    let mut cls_mut = cls_ptr as *mut ExternType;
                    loop {
                        if unsafe { (*cls_mut).indexer.is_some() } {
                            let indexer = unsafe { (*cls_mut).indexer.as_ref().unwrap() };
                            self.constraint_solver_unify(
                                constraint,
                                index_type,
                                indexer.index_type,
                            );
                            parts.insert_type_id(indexer.index_result_type);
                            break;
                        }

                        if let Some(parent) = unsafe { (*cls_mut).parent } {
                            cls_mut = unsafe {
                                crate::functions::get_type_alt_j::get_type_id::<ExternType>(parent)
                                    as *mut ExternType
                            };
                            continue;
                        }

                        break;
                    }
                }
            }

            let scope = unsafe { (*constraint).scope };
            let location = unsafe { (*constraint).location };

            let res =
                self.simplify_intersection_not_null_scope_location_type_ids(scope, location, parts);
            self.constraint_solver_unify(constraint, rhs_type, res);
        }

        // Other types do not support index assignment.
        self.bind_not_null_constraint_type_id_type_id(constraint, c.prop_type, unsafe {
            (*self.builtin_types).errorType
        });

        true
    }
}
