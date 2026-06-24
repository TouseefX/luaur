use crate::functions::get_type_alt_j::get_type_id;
use crate::records::negation_type::NegationType;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use alloc::string::{String, ToString};

impl Unifier {
    pub fn unifier_try_unify_negations(&mut self, sub_ty: TypeId, super_ty: TypeId) {
        if unsafe { get_type_id::<NegationType>(sub_ty) }.is_null()
            && unsafe { get_type_id::<NegationType>(super_ty) }.is_null()
        {
            self.ice_string("tryUnifyNegations superTy or subTy must be a negation type");
        }

        let sub_norm = unsafe { (*self.normalizer).normalize(sub_ty) };
        let super_norm = unsafe { (*self.normalizer).normalize(super_ty) };

        let mut state = self.unifier_make_child_unifier();
        state.unifier_try_unify_normalized_types(
            sub_ty,
            super_ty,
            &sub_norm,
            &super_norm,
            String::new(),
            None,
        );
        if state.errors.is_empty() {
            let context = self.unifier_mismatch_context();
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::TypeMismatch(TypeMismatch {
                    wanted_type: super_ty,
                    given_type: sub_ty,
                    reason: String::new(),
                    error: None,
                    context,
                }),
            );
        }
    }
}
