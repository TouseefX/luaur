//! Source: `Analysis/include/Luau/TypePath.h:115` (hand-ported)
use crate::type_aliases::type_pack_id::TypePackId;

#[allow(non_snake_case)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct GenericPackMapping {
    pub mappedType: TypePackId,
}

impl GenericPackMapping {
    pub fn operator_eq(&self, other: &GenericPackMapping) -> bool {
        self.mappedType == other.mappedType
    }
}
