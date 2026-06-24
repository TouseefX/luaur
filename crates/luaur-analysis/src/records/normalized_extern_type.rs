use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct NormalizedExternType {
    pub(crate) extern_types: BTreeMap<TypeId, TypeIds>,
    pub(crate) shape_extensions: TypeIds,
    pub(crate) ordering: Vec<TypeId>,
}
