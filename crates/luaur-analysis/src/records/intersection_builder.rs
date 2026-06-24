use crate::records::builtin_types::BuiltinTypes;
use crate::records::type_arena::TypeArena;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;

#[derive(Debug, Clone)]
pub struct IntersectionBuilder {
    pub(crate) arena: *mut TypeArena,
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) parts: TypeIds,
    pub(crate) is_bottom: bool,
}
