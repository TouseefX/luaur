//! Faithful port of `TypeChecker2::stripFromNilAndReport` (TypeChecker2.cpp:1883-1910).
use crate::enums::value::Value;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_nil::is_nil;
use crate::functions::should_suppress_errors_type_utils::should_suppress_errors;
use crate::records::normalization_too_complex::NormalizationTooComplex;
use crate::records::optional_value_access::OptionalValueAccess;
use crate::records::type_checker_2::TypeChecker2;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn strip_from_nil_and_report(&mut self, ty: TypeId, location: &Location) -> TypeId {
        let ty = unsafe { follow_type_id(ty) };

        // if (auto utv = get<UnionType>(ty))
        //     if (!std::any_of(begin(utv), end(utv), isNil)) return ty;
        let utv = unsafe { get_type_id::<UnionType>(ty) };
        if !utv.is_null() {
            let utv = unsafe { &*utv };
            if !utv.options.iter().any(|&opt| is_nil(opt)) {
                return ty;
            }
        }

        if let Some(stripped_union) = self.try_strip_union_from_nil(ty) {
            match Value::from(should_suppress_errors(&mut self.normalizer, ty)) {
                Value::Suppress => {}
                Value::NormalizationFailed => {
                    self.report_error_type_error_data_location(
                        NormalizationTooComplex::default().into(),
                        location,
                    );
                    // [[fallthrough]]
                    self.report_error_type_error_data_location(
                        OptionalValueAccess { optional: ty }.into(),
                        location,
                    );
                }
                Value::DoNotSuppress => {
                    self.report_error_type_error_data_location(
                        OptionalValueAccess { optional: ty }.into(),
                        location,
                    );
                }
            }

            return unsafe { follow_type_id(stripped_union) };
        }

        ty
    }
}
