//! C++ `RefineTypeScrubber::ignoreChildren(TypeId ty)`
//! (BuiltinTypeFunctions.cpp:1126-1129): `return !is<UnionType,
//! IntersectionType>(ty);`
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::intersection_type::IntersectionType;
use crate::records::refine_type_scrubber::RefineTypeScrubber;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;

impl RefineTypeScrubber {
    pub fn ignore_children_type_id(&mut self, ty: TypeId) -> bool {
        let is_union = !unsafe { get_type_id::<UnionType>(ty) }.is_null();
        let is_intersection = !unsafe { get_type_id::<IntersectionType>(ty) }.is_null();
        !(is_union || is_intersection)
    }
}
