use crate::functions::get_table_type::get_table_type;
use crate::records::table_type::TableType;
use crate::type_aliases::type_id::TypeId;

pub fn get_mutable_table_type(ty: TypeId) -> *mut TableType {
    if let Some(table) = get_table_type(ty) {
        table as *const TableType as *mut TableType
    } else {
        core::ptr::null_mut()
    }
}
