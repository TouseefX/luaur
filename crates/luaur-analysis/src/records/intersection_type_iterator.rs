//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/Type.h:1122:intersection_type_iterator`
//! Source: `Analysis/include/Luau/Type.h:1122-1124` (hand-ported)

use crate::records::intersection_type::IntersectionType;
use crate::records::type_iterator::TypeIterator;

pub type IntersectionTypeIterator = TypeIterator<IntersectionType>;
