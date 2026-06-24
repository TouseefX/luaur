use crate::functions::get_type_alt_j::get_type_id;
use crate::records::never_type::NeverType;
use crate::records::normalized_type::NormalizedType;
use luaur_common::FFlag;

impl NormalizedType {
    pub fn has_integers(&self) -> bool {
        if FFlag::LuauIntegerType2.get() {
            unsafe { get_type_id::<NeverType>(self.integers).is_null() }
        } else {
            false
        }
    }
}
