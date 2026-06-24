//! Source: `Analysis/src/Unifier.cpp` (Unifier::tryUnifyScalarShape, L2151-2224)
use crate::enums::normalization_result::NormalizationResult;
use crate::enums::table_state::TableState;
use crate::functions::get_metatable_type::get_metatable_type_id_not_null_builtin_types;
use crate::functions::has_unification_too_complex::has_unification_too_complex;
use crate::records::r#type::Type;
use crate::records::table_type::TableType;
use crate::records::type_error::TypeError;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_variant::TypeVariant;
use alloc::string::String;
use alloc::sync::Arc;

impl Unifier {
    /// `void Unifier::tryUnifyScalarShape(TypeId subTy, TypeId superTy, bool reversed)`
    pub fn unifier_try_unify_scalar_shape(
        &mut self,
        mut sub_ty: TypeId,
        mut super_ty: TypeId,
        reversed: bool,
    ) {
        let osub_ty = sub_ty;
        let osuper_ty = super_ty;

        // If the normalizer hits resource limits, we can't show it's uninhabited, so, we should continue.
        if self.check_inhabited
            && unsafe { (*self.normalizer).is_inhabited_type_id(sub_ty) }
                == NormalizationResult::False
        {
            return;
        }

        if reversed {
            core::mem::swap(&mut sub_ty, &mut super_ty);
        }

        let super_table = self.log.txn_log_get_mutable::<TableType, TypeId>(super_ty);

        if super_table.is_null() || unsafe { (*super_table).state } != TableState::Free {
            let context = self.unifier_mismatch_context();
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::TypeMismatch(TypeMismatch {
                    wanted_type: osuper_ty,
                    given_type: osub_ty,
                    reason: String::new(),
                    error: None,
                    context,
                }),
            );
            return;
        }

        // Given t1 where t1 = { lower: (t1) -> (a, b...) }
        // It should be the case that `string <: t1` iff `(subtype's metatable).__index <: t1`
        if let Some(metatable) =
            get_metatable_type_id_not_null_builtin_types(sub_ty, unsafe { &*self.builtin_types })
        {
            let mttv = self.log.txn_log_get::<TableType, TypeId>(metatable);
            if mttv.is_null() {
                self.scalar_shape_fail(osuper_ty, osub_ty, None);
            }

            if let Some(prop) = unsafe { (*mttv).props.get("__index") } {
                let ty = prop.type_deprecated();
                let mut child = self.unifier_make_child_unifier();
                child.try_unify_type_id_type_id_bool_bool_literal_properties(
                    ty, super_ty, false, false, None,
                );

                // To perform subtype <: free table unification, we have tried to unify (subtype's metatable) <: free table
                // There is a chance that it was unified with the original subtype, but then, (subtype's metatable) <: subtype could've failed
                // Here we check if we have a new supertype instead of the original free table and try original subtype <: new supertype check
                let new_super_ty = child.log.follow_type_id(super_ty);

                if super_ty != new_super_ty
                    && self
                        .can_unify_type_id_type_id(sub_ty, new_super_ty)
                        .is_empty()
                {
                    self.log
                        .replace_type_id_t(super_ty, Type::new(TypeVariant::Bound(sub_ty)));
                    return;
                }

                if let Some(e) = has_unification_too_complex(&child.errors) {
                    self.report_error_type_error(e);
                } else if !child.errors.is_empty() {
                    let first = child.errors[0].clone();
                    self.scalar_shape_fail(osuper_ty, osub_ty, Some(first));
                }

                let child_errors_empty = child.errors.is_empty();
                self.log.concat(child.log);

                // To perform subtype <: free table unification, we have tried to unify (subtype's metatable) <: free table
                // We return success because subtype <: free table which means that correct unification is to replace free table with the subtype
                if child_errors_empty {
                    self.log
                        .replace_type_id_t(super_ty, Type::new(TypeVariant::Bound(sub_ty)));
                }

                return;
            } else {
                self.scalar_shape_fail(osuper_ty, osub_ty, None);
                return;
            }
        }

        let context = self.unifier_mismatch_context();
        self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::TypeMismatch(TypeMismatch {
                wanted_type: osuper_ty,
                given_type: osub_ty,
                reason: String::new(),
                error: None,
                context,
            }),
        );
    }

    /// The `fail` lambda from `tryUnifyScalarShape`.
    fn scalar_shape_fail(&mut self, osuper_ty: TypeId, osub_ty: TypeId, e: Option<TypeError>) {
        let reason = String::from("The given type's metatable does not satisfy the requirements.");
        let context = self.unifier_mismatch_context();
        self.report_error_location_type_error_data(
            self.location,
            TypeErrorData::TypeMismatch(TypeMismatch {
                wanted_type: osuper_ty,
                given_type: osub_ty,
                reason,
                error: e.map(Arc::new),
                context,
            }),
        );
    }
}
