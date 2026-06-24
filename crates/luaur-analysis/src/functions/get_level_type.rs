use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::table_type::TableType;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;

pub fn get_level(ty: TypeId) -> Option<&'static TypeLevel> {
    let ty = unsafe { follow_type_id(ty) };

    unsafe {
        if let Some(ftv) = get_type_id::<FreeType>(ty).as_ref() {
            Some(&ftv.level)
        } else if let Some(ttv) = get_type_id::<TableType>(ty).as_ref() {
            Some(&ttv.level)
        } else if let Some(ftv) = get_type_id::<FunctionType>(ty).as_ref() {
            Some(&ftv.level)
        } else {
            None
        }
    }
}

#[allow(unused_imports)]
pub use get_level as get_level_type_id;
