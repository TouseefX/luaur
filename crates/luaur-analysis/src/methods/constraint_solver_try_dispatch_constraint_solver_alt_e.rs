use crate::enums::polarity::Polarity;
use crate::enums::table_state::TableState;
use crate::functions::extend_type_pack::extend_type_pack;
use crate::functions::follow_type::follow_type_id;
use crate::functions::fresh_type::fresh_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::track_interior_free_type::track_interior_free_type;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::iterable_constraint::IterableConstraint;
use crate::records::table_indexer::TableIndexer;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::props_type::Props;

impl ConstraintSolver {
    pub fn try_dispatch_iterable_constraint_not_null_constraint_bool(
        &mut self,
        c: &IterableConstraint,
        constraint: *const Constraint,
        force: bool,
    ) -> bool {
        let iterator = unsafe {
            extend_type_pack(
                &mut *self.arena,
                self.builtin_types,
                c.iterator,
                3,
                alloc::vec::Vec::new(),
            )
        };

        if iterator.head.len() < 3 {
            if let Some(tail) = iterator.tail {
                if self.is_blocked_type_pack_id(tail) {
                    return if force {
                        true
                    } else {
                        self.block_type_pack_id_not_null_constraint(tail, constraint)
                    };
                }
            }
        }

        let mut blocked = false;
        for ty in &iterator.head {
            if self.is_blocked_type_id(*ty) {
                self.block_type_id_not_null_constraint(*ty, constraint);
                blocked = true;
            }
        }

        if blocked {
            return false;
        }

        if iterator.head.is_empty() {
            for ty in &c.variables {
                self.bind_not_null_constraint_type_id_type_id(constraint, *ty, unsafe {
                    (*self.builtin_types).errorType
                });
            }
            return true;
        }

        let next_ty = unsafe { follow_type_id(iterator.head[0]) };
        if unsafe { !get_type_id::<FreeType>(next_ty).is_null() } {
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
                        TableState::Free,
                    ),
                )
            };
            track_interior_free_type(scope, table_ty);

            self.constraint_solver_unify(constraint, next_ty, table_ty);

            let mut it = c.variables.iter();
            if let Some(ty) = it.next() {
                self.bind_not_null_constraint_type_id_type_id(constraint, *ty, key_ty);
            }
            if let Some(ty) = it.next() {
                self.bind_not_null_constraint_type_id_type_id(constraint, *ty, value_ty);
            }
            for ty in it {
                self.bind_not_null_constraint_type_id_type_id(constraint, *ty, unsafe {
                    (*self.builtin_types).nilType
                });
            }

            return true;
        }

        if unsafe { !get_type_id::<FunctionType>(next_ty).is_null() } {
            let table_ty = if iterator.head.len() >= 2 {
                iterator.head[1]
            } else {
                unsafe { (*self.builtin_types).nilType }
            };

            return self.try_dispatch_iterable_function(next_ty, table_ty, c, constraint);
        }

        self.try_dispatch_iterable_table(iterator.head[0], c, constraint, force)
    }
}
