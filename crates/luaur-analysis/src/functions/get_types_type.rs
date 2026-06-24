use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

pub fn get_types_union_type(utv: *const UnionType) -> &'static [TypeId] {
    let utv_ref = unsafe { &*utv };
    &utv_ref.options
}
