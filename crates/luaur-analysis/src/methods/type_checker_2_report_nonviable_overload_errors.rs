use crate::records::multiple_nonviable_overloads::MultipleNonviableOverloads;
use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn report_nonviable_overload_errors(
        &mut self,
        nonviable_overloads: alloc::vec::Vec<(TypeId, ErrorVec)>,
        call_func_location: Location,
        arg_head_size: usize,
        call_location: Location,
    ) -> bool {
        let mut reported_errors: Option<ErrorVec> = None;
        let mut multiple_overloads_have_errors = false;

        for (ty, errs) in &nonviable_overloads {
            if !self.is_error_suppressing_location_type_id(call_func_location, *ty)
                && !errs.is_empty()
            {
                if reported_errors.is_some() {
                    multiple_overloads_have_errors = true;
                    break;
                }
                reported_errors = Some(errs.clone());
            }
        }

        if multiple_overloads_have_errors {
            self.report_error_type_error_data_location(
                TypeErrorData::MultipleNonviableOverloads(MultipleNonviableOverloads {
                    attempted_arg_count: arg_head_size,
                }),
                &call_location,
            );
            true
        } else if let Some(errors) = reported_errors {
            self.report_errors(errors);
            true
        } else {
            false
        }
    }
}
