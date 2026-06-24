use crate::functions::has_unification_too_complex::has_unification_too_complex;
use crate::records::type_mismatch::TypeMismatch;
use crate::records::unifier::Unifier;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;

impl Unifier {
    pub fn check_child_unifier_type_mismatch_error_vec_type_id_type_id(
        &mut self,
        inner_errors: &ErrorVec,
        wanted_type: TypeId,
        given_type: TypeId,
    ) {
        if let Some(e) = has_unification_too_complex(inner_errors) {
            self.report_error_type_error(e);
        } else if !inner_errors.is_empty() {
            let context = self.unifier_mismatch_context();
            self.report_error_location_type_error_data(
                self.location,
                TypeErrorData::TypeMismatch(TypeMismatch {
                    wanted_type,
                    given_type,
                    reason: alloc::string::String::new(),
                    error: None,
                    context,
                }),
            );
        }
    }
}
