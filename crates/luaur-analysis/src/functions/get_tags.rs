use crate::functions::follow_type::follow_type_id;
use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::records::extern_type::ExternType;
use crate::records::function_type::FunctionType;
use crate::records::table_type::TableType;
use crate::type_aliases::tags::Tags;
use crate::type_aliases::type_id::TypeId;

pub fn get_tags(ty: TypeId) -> Option<&'static mut Tags> {
    let ty = unsafe { follow_type_id(ty) };

    unsafe {
        if let Some(ftv) = get_mutable_type_id::<FunctionType>(ty).as_mut() {
            Some(&mut ftv.tags)
        } else if let Some(ttv) = get_mutable_type_id::<TableType>(ty).as_mut() {
            Some(&mut ttv.tags)
        } else if let Some(etv) = get_mutable_type_id::<ExternType>(ty).as_mut() {
            Some(&mut etv.tags)
        } else {
            None
        }
    }
}
