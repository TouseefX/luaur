#[derive(Debug, Clone)]
pub struct TraversalState {
    pub(crate) current: crate::type_aliases::type_or_pack::TypeOrPack,
    pub(crate) builtin_types: *const crate::records::builtin_types::BuiltinTypes,
    pub(crate) arena: *mut crate::records::type_arena::TypeArena,
    pub(crate) steps: i32,
    pub(crate) encountered_error_suppression: bool,
}
