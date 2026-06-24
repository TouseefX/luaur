use crate::functions::get_type_alt_j::get_type_id;
use crate::records::any_type::AnyType;
use crate::records::normalized_type::NormalizedType;

impl NormalizedType {
    pub fn should_suppress_errors(&self) -> bool {
        self.has_errors() || !unsafe { get_type_id::<AnyType>(self.tops).is_null() }
    }
}
