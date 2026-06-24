use crate::records::scope::Scope;
use crate::records::type_level::TypeLevel;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct Quantifier {
    pub base: TypeOnceVisitor,
    pub level: TypeLevel,
    pub generics: Vec<TypeId>,
    pub generic_packs: Vec<TypePackId>,
    pub scope: *mut Scope,
    pub seen_generic_type: bool,
    pub seen_mutable_type: bool,
}
