use crate::functions::get_singleton_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::normalized_string_type::NormalizedStringType;
use crate::records::singleton_type::SingletonType;
use crate::records::string_singleton::StringSingleton;

pub fn is_normalized_string(ty: &NormalizedStringType) -> bool {
    if ty.is_string() {
        return true;
    }

    for (str, &type_id) in &ty.singletons {
        unsafe {
            let stv = get_type_id::<SingletonType>(type_id);
            if stv.is_null() {
                return false;
            }

            let sstv = get_singleton_type::<StringSingleton>(stv);
            if sstv.is_null() {
                return false;
            }

            if (*sstv).value != *str {
                return false;
            }
        }
    }

    true
}
