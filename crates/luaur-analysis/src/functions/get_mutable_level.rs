use crate::functions::get_level_type::get_level;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;

pub fn get_mutable_level(ty: TypeId) -> *mut TypeLevel {
    if let Some(level) = get_level(ty) {
        level as *const TypeLevel as *mut TypeLevel
    } else {
        core::ptr::null_mut()
    }
}
