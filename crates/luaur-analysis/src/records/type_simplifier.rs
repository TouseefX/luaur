use crate::records::builtin_types::BuiltinTypes;
use crate::records::property_type_path::Property;
use crate::records::type_arena::TypeArena;
use crate::records::type_ids::TypeIds;
use crate::type_aliases::type_id::TypeId;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct TypeSimplifier {
    pub(crate) builtin_types: *const BuiltinTypes,
    pub(crate) arena: *const TypeArena,
    pub(crate) blocked_types: DenseHashSet<TypeId>,
    pub(crate) recursion_depth: i32,
}
