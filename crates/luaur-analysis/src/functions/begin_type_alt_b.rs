use crate::records::intersection_type::IntersectionType;
use crate::type_aliases::intersection_type_iterator::IntersectionTypeIterator;

pub fn begin_intersection_type(_itv: &IntersectionType) -> IntersectionTypeIterator {
    IntersectionTypeIterator::default()
}
