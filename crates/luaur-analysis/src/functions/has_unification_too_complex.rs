use crate::functions::get_error::get_type_error;
use crate::records::type_error::TypeError;
use crate::records::unification_too_complex::UnificationTooComplex;
use crate::type_aliases::error_vec::ErrorVec;

pub fn has_unification_too_complex(errors: &ErrorVec) -> Option<TypeError> {
    let mut found: Option<TypeError> = None;

    for te in errors.iter() {
        let unification = unsafe { get_type_error::<UnificationTooComplex>(te) };
        if !unification.is_null() {
            found = Some(te.clone());
            break;
        }
    }

    found
}
