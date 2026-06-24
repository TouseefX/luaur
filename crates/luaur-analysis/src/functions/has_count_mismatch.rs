use crate::functions::get_error::get_type_error;
use crate::records::count_mismatch::CountMismatch;
use crate::records::type_error::TypeError;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_error_data::TypeErrorDataMember;

/// C++ `hasCountMismatch(const ErrorVec& errors)`.
pub fn has_count_mismatch(errors: &ErrorVec) -> Option<TypeError> {
    for te in errors.iter() {
        let ptr = unsafe { get_type_error::<CountMismatch>(te) };
        if !ptr.is_null() {
            return Some(te.clone());
        }
    }
    None
}
