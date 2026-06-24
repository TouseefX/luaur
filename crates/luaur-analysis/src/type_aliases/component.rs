//! Node: `cxx:TypeAlias:Luau.Analysis:Analysis/include/Luau/TypePath.h:124:component`
//! Source: `Analysis/include/Luau/TypePath.h` (TypePath.h:124, hand-ported)

use crate::enums::pack_field::PackField;
use crate::enums::type_field::TypeField;
use crate::records::generic_pack_mapping::GenericPackMapping;
use crate::records::index::Index;
use crate::records::pack_slice::PackSlice;
use crate::records::property_type_path::Property;
use crate::records::reduction::Reduction;

// C++: using Component = Luau::Variant<Property, Index, TypeField, PackField,
//                                      PackSlice, Reduction, GenericPackMapping>;
// 7 alternatives — dedicated enum per the Type-SCC convention.
#[derive(Debug, Clone, PartialEq)]
pub enum Component {
    Property(Property),
    Index(Index),
    TypeField(TypeField),
    PackField(PackField),
    PackSlice(PackSlice),
    Reduction(Reduction),
    GenericPackMapping(GenericPackMapping),
}
