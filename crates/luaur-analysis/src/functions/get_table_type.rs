use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

pub fn get_table_type(type_id: TypeId) -> Option<&'static TableType> {
    let mut ty = unsafe { follow_type_id(type_id) };

    let ttv_ptr = unsafe { get_type_id::<TableType>(ty) };
    if !ttv_ptr.is_null() {
        return Some(unsafe { &*ttv_ptr });
    }

    let mtv_ptr = unsafe { get_type_id::<MetatableType>(ty) };
    if !mtv_ptr.is_null() {
        let mtv = unsafe { &*mtv_ptr };
        ty = unsafe { follow_type_id(mtv.table()) };

        let ttv_ptr = unsafe { get_type_id::<TableType>(ty) };
        if !ttv_ptr.is_null() {
            return Some(unsafe { &*ttv_ptr });
        }
    }

    None
}
