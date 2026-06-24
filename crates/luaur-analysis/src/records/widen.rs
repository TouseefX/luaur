use crate::records::builtin_types::BuiltinTypes;
use crate::records::substitution::Substitution;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct Widen {
    pub(crate) base: Substitution,
    pub(crate) builtin_types: *const BuiltinTypes,
}
