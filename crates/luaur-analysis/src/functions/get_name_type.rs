use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

pub fn get_name(type_id: TypeId) -> Option<&'static str> {
    let mut ty = unsafe { follow_type_id(type_id) };

    let mtv_ptr = unsafe { get_type_id::<MetatableType>(ty) };
    if !mtv_ptr.is_null() {
        let mtv = unsafe { &*mtv_ptr };
        if let Some(name) = mtv.syntheticName() {
            return Some(Box::leak(name.to_string().into_boxed_str()));
        }
        ty = unsafe { follow_type_id(mtv.table()) };
    }

    let ttv_ptr = unsafe { get_type_id::<TableType>(ty) };
    if !ttv_ptr.is_null() {
        let ttv = unsafe { &*ttv_ptr };
        if let Some(name) = &ttv.name {
            return Some(Box::leak(name.clone().into_boxed_str()));
        }
        if let Some(name) = &ttv.synthetic_name {
            return Some(Box::leak(name.clone().into_boxed_str()));
        }
    }

    None
}
