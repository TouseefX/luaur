use crate::records::substitution::Substitution;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct ApplyTypeFunction {
    pub(crate) base: Substitution,
    pub(crate) encountered_forwarded_type: bool,
    pub(crate) type_arguments: DenseHashMap<TypeId, TypeId>,
    pub(crate) type_pack_arguments: DenseHashMap<TypePackId, TypePackId>,
}
