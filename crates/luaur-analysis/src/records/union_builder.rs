use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct UnionBuilder {
    pub(crate) arena: *mut TypeArena,
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) options: TypeIds,
    pub(crate) is_top: bool,
}
