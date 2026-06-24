use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
    pub fn has_errors(&self) -> bool {
        unsafe { get_type_id::<NeverType>(self.errors).is_null() }
    }
}
