use crate::functions::get_type::get_singleton_type;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::boolean_singleton::BooleanSingleton;
use crate::records::never_type::NeverType;
use crate::records::primitive_type::PrimitiveType;
use crate::records::singleton_type::SingletonType;
use crate::type_aliases::type_id::TypeId;

pub fn is_normalized_boolean(ty: TypeId) -> bool {
    unsafe {
        if get_type_id::<NeverType>(ty).is_null() {
            return true;
        } else if let Some(ptv) = get_type_id::<PrimitiveType>(ty).as_ref() {
            return ptv.r#type == PrimitiveType::Boolean;
        } else if let Some(stv) = get_type_id::<SingletonType>(ty).as_ref() {
            return !get_singleton_type::<BooleanSingleton>(stv as *const SingletonType).is_null();
        } else {
            return false;
        }
    }
}
