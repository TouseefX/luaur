use crate::records::substitution::Substitution;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct Replacer {
    pub(crate) base: Substitution,
    pub(crate) replacements: *mut DenseHashMap<TypeId, TypeId>,
    pub(crate) replacement_packs: *mut DenseHashMap<TypePackId, TypePackId>,
}
