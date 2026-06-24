use crate::records::type_pack_var::TypePackVar;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn as_mutable_type_pack(tp: TypePackId) -> *mut TypePackVar {
    tp as *const TypePackVar as *mut TypePackVar
}
