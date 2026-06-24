use crate::enums::refinements_op_kind::RefinementsOpKind;
use crate::enums::value::Value;
use crate::functions::must_defer_intersection::must_defer_intersection;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::constraint_generator::ConstraintGenerator;
use crate::records::constraint_generator::ConstraintGenerator as _ConstraintGenerator;
use crate::records::error_suppression::ErrorSuppression;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::type_aliases::constraint_v::ConstraintV;
use crate::type_aliases::refinement_context::RefinementContext;
use crate::type_aliases::refinement_id_refinement::RefinementId;
use crate::type_aliases::scope_ptr_constraint_generator::ScopePtr;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl ConstraintGenerator {
    pub fn apply_refinements(
        &mut self,
        scope: &ScopePtr,
        location: Location,
        refinement: RefinementId,
    ) {
        if refinement.is_null() {
            return;
        }

        let mut refinements: RefinementContext = RefinementContext::default();
        let mut constraints: Vec<ConstraintV> = Vec::new();

        self.compute_refinement(
            scope.as_ref() as *const _ as *mut _,
            location,
            refinement,
            &mut refinements,
            true,
            false,
            &mut constraints,
        );

        let mut flush_constraints = |this: &mut ConstraintGenerator,
                                     kind: RefinementsOpKind,
                                     ty: TypeId,
                                     discriminants: &mut Vec<TypeId>|
         -> TypeId {
            if discriminants.is_empty() {
                return ty;
            }

            if kind == RefinementsOpKind::None {
                LUAU_ASSERT!(false);
                return ty;
            }

            let mut args = Vec::new();
            args.push(ty);

            let builtin_types = this.builtin_types;
            let type_functions = unsafe { &(*builtin_types).typeFunctions };

            let func = if kind == RefinementsOpKind::Intersect {
                &type_functions.intersect_func
            } else {
                &type_functions.refine_func
            };

            LUAU_ASSERT!(!func.name.is_empty());
            args.extend_from_slice(discriminants.as_slice());

            let result_type =
                this.create_type_function_instance(func, args, Vec::new(), scope, location);
            discriminants.clear();
            result_type
        };

        let scope_raw = scope.as_ref() as *const _ as *mut _;

        for (def, partition) in refinements.iter() {
            let def_ty = self.lookup(scope, location, *def, false);
            let Some(def_ty) = def_ty else { continue };

            let mut ty = def_ty;

            let mut discriminants: Vec<TypeId> = Vec::new();
            let mut kind = RefinementsOpKind::None;

            let mut must_defer = must_defer_intersection(ty);

            for dt in &partition.discriminant_types {
                let dt_val = *dt;

                must_defer = must_defer || must_defer_intersection(dt_val);

                if must_defer {
                    if kind == RefinementsOpKind::Intersect {
                        ty = flush_constraints(self, kind, ty, &mut discriminants);
                    }
                    kind = RefinementsOpKind::Refine;
                    discriminants.push(dt_val);
                } else {
                    let status: ErrorSuppression = should_suppress_errors(self.normalizer, ty);

                    if status.value == Value::NormalizationFailed {
                        self.report_error(location, NormalizationTooComplex::default().into());
                    }

                    if kind == RefinementsOpKind::Refine {
                        ty = flush_constraints(self, kind, ty, &mut discriminants);
                    }
                    kind = RefinementsOpKind::Intersect;

                    discriminants.push(dt_val);

                    if status.value == Value::Suppress {
                        ty = flush_constraints(self, kind, ty, &mut discriminants);
                        ty = self.make_union_scope_ptr_location_type_id_type_id(
                            scope_raw,
                            location,
                            ty,
                            unsafe { (*self.builtin_types).errorType },
                        );
                    }
                }
            }

            if kind != RefinementsOpKind::None {
                ty = flush_constraints(self, kind, ty, &mut discriminants);
            }

            if partition.should_append_nil_type {
                ty = self.create_type_function_instance(
                    unsafe { &(*self.builtin_types).typeFunctions.weakoptional_func },
                    vec![ty],
                    Vec::new(),
                    scope,
                    location,
                );
            }

            self.update_r_value_refinements_scope_ptr_def_id_type_id(scope, *def, ty);
        }

        for c in constraints {
            self.add_constraint_scope_ptr_location_constraint_v(scope, location, c);
        }
    }
}
