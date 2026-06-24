use crate::records::builtin_types::BuiltinTypes;
use crate::records::scope::Scope;
use crate::records::substitution::Substitution;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct ReplaceGenerics {
    pub base: Substitution,
    pub builtin_types: *mut BuiltinTypes,
    pub level: TypeLevel,
    pub scope: *mut Scope,
    pub generics: Vec<TypeId>,
    pub generic_packs: Vec<TypePackId>,
}
