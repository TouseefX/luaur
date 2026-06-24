use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::type_id::TypeId;

pub fn get_types_intersection_type(t: *const core::ffi::c_void) -> &'static [TypeId] {
    let itv = unsafe { &*(t as *const IntersectionType) };
    &itv.parts
}
