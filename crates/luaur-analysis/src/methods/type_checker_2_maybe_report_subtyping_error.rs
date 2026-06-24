//! Faithful port of `TypeChecker2::maybeReportSubtypingError`
//! (TypeChecker2.cpp:3403-3419).
use crate::enums::context_error::Context;
use crate::enums::value::Value;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::type_mismatch::TypeMismatch;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn maybe_report_subtyping_error(
        &mut self,
        sub_ty: TypeId,
        super_ty: TypeId,
        location: &Location,
    ) {
        let suppression = should_suppress_errors(&mut self.normalizer, sub_ty)
            .or_else(&should_suppress_errors(&mut self.normalizer, super_ty));

        match Value::from(suppression) {
            Value::Suppress => return,
            Value::NormalizationFailed => {
                self.report_error_type_error_data_location(
                    NormalizationTooComplex::default().into(),
                    location,
                );
            }
            Value::DoNotSuppress => {}
        }

        // reportError(TypeMismatch{superTy, subTy}, location);
        self.report_error_type_error_data_location(
            TypeErrorData::TypeMismatch(TypeMismatch {
                wanted_type: super_ty,
                given_type: sub_ty,
                reason: alloc::string::String::new(),
                error: None,
                context: Context::Covariant,
            }),
            location,
        );
    }
}
