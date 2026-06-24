use alloc::sync::Arc;

use crate::enums::polarity::Polarity;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::as_mutable_type_pack::as_mutable_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_pack::follow_type_pack_id;
use crate::functions::generalize::generalize;
use crate::functions::generalize_type::generalize_type;
use crate::functions::generalize_type_pack::generalize_type_pack;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::get_type_pack::get_type_pack_id;
use crate::functions::is_known::is_known;
use crate::functions::prune_unnecessary_generics::prune_unnecessary_generics;
use crate::functions::seal_table::seal_table;
use crate::records::blocked_type::BlockedType;
use crate::records::code_too_complex::CodeTooComplex;
use crate::records::constraint::Constraint;
use crate::records::constraint_solver::ConstraintSolver;
use crate::records::free_type::FreeType;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::function_type::FunctionType;
use crate::records::generalization_constraint::GeneralizationConstraint;
use crate::records::generalization_params::GeneralizationParams;
use crate::records::pending_expansion_type::PendingExpansionType;
use crate::records::r#type::Type;
use crate::records::table_type::TableType;
use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use crate::type_aliases::type_variant::TypeVariant;

impl ConstraintSolver {
    pub fn try_dispatch_generalization_constraint_not_null_constraint(
        &mut self,
        c: &GeneralizationConstraint,
        constraint: *const Constraint,
    ) -> bool {
        let generalized_type = unsafe { follow_type_id(c.generalized_type) };

        if self.is_blocked_type_id(c.source_type) {
            return self.block_type_id_not_null_constraint(c.source_type, constraint);
        } else if unsafe { !get_type_id::<PendingExpansionType>(generalized_type).is_null() } {
            return self.block_type_id_not_null_constraint(generalized_type, constraint);
        }

        let generalized_ty = generalize(
            self.arena,
            self.builtin_types,
            unsafe { (*constraint).scope },
            &mut self.generalized_types_ as *mut _,
            c.source_type,
            None,
        );

        if generalized_ty.is_none() {
            self.report_error_type_error_data_location(CodeTooComplex::default().into(), unsafe {
                &(*constraint).location
            });
        }

        if let Some(generalized_ty) = generalized_ty {
            prune_unnecessary_generics(
                self.arena,
                self.builtin_types,
                unsafe { (*constraint).scope },
                &mut self.generalized_types_ as *mut _,
                generalized_ty,
            );

            if unsafe { !get_type_id::<BlockedType>(generalized_type).is_null() } {
                self.bind_not_null_constraint_type_id_type_id(
                    constraint,
                    generalized_type,
                    generalized_ty,
                );
            } else {
                self.constraint_solver_unify(constraint, generalized_type, generalized_ty);
            }

            let fty =
                unsafe { get_mutable_type_id::<FunctionType>(follow_type_id(generalized_type)) };
            if !fty.is_null() && c.has_deprecated_attribute {
                unsafe {
                    (*fty).is_deprecated_function = true;
                    (*fty).deprecated_info = Some(Arc::new(c.deprecated_info.clone()));
                }
            }
        } else {
            self.report_error_type_error_data_location(CodeTooComplex::default().into(), unsafe {
                &(*constraint).location
            });
            self.bind_not_null_constraint_type_id_type_id(constraint, c.generalized_type, unsafe {
                (*self.builtin_types).errorType
            });
        }

        unsafe {
            let scope = (*constraint).scope;

            if let Some(interior_free_types) = (*scope).interior_free_types.as_ref() {
                let interior_free_types = interior_free_types.clone();
                for ty in interior_free_types {
                    let ty = follow_type_id(ty);
                    let free_ty = get_type_id::<FreeType>(ty);

                    if !free_ty.is_null() {
                        let params = GeneralizationParams {
                            found_outside_functions: true,
                            use_count: 1,
                            polarity: (*free_ty).polarity,
                        };
                        let res =
                            generalize_type(self.arena, self.builtin_types, scope, ty, &params);
                        if res.resource_limits_exceeded {
                            self.report_error_type_error_data_location(
                                CodeTooComplex::default().into(),
                                &(*scope).location,
                            );
                        }
                    } else if !get_type_id::<TableType>(ty).is_null() {
                        seal_table(scope, ty);
                    }

                    self.unblock_type_id_location(ty, (*constraint).location);
                }
            }

            if let Some(interior_free_type_packs) = (*scope).interior_free_type_packs.as_ref() {
                let interior_free_type_packs = interior_free_type_packs.clone();
                for tp in interior_free_type_packs {
                    let tp = follow_type_pack_id(tp);
                    let free_tp = get_type_pack_id::<FreeTypePack>(tp);

                    if !free_tp.is_null() {
                        let params = GeneralizationParams {
                            found_outside_functions: true,
                            use_count: 1,
                            polarity: (*free_tp).polarity,
                        };
                        luaur_common::macros::luau_assert::LUAU_ASSERT!(is_known(params.polarity));
                        generalize_type_pack(self.arena, self.builtin_types, scope, tp, &params);
                    }
                }
            }

            if c.no_generics {
                let ft = get_mutable_type_id::<FunctionType>(c.source_type);
                if !ft.is_null() {
                    for gen in (*ft).generics.iter().copied() {
                        (*as_mutable_type_id(gen)).ty =
                            TypeVariant::Bound((*self.builtin_types).unknownType);
                    }
                    (*ft).generics.clear();

                    for gen in (*ft).generic_packs.iter().copied() {
                        (*as_mutable_type_pack_id(gen)).ty =
                            TypePackVariant::Bound((*self.builtin_types).unknownTypePack);
                    }
                    (*ft).generic_packs.clear();
                }
            }
        }

        true
    }
}
