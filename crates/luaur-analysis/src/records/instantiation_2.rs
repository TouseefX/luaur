use crate::records::scope::Scope;
use crate::records::substitution::Substitution;
use crate::records::subtyping::Subtyping;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct Instantiation2 {
    pub(crate) base: Substitution,
    pub(crate) generic_substitutions: DenseHashMap<TypeId, TypeId>,
    pub(crate) generic_pack_substitutions: DenseHashMap<TypePackId, TypePackId>,
    pub(crate) subtyping: *mut Subtyping,
    pub(crate) scope: *mut Scope,
}
