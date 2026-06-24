#[derive(Debug, Clone)]
pub struct TypeFunctionSerializer {
    pub(crate) state:
        *mut crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState,
    pub(crate) type_function_runtime:
        *mut crate::records::type_function_runtime::TypeFunctionRuntime,
    pub(crate) queue: alloc::vec::Vec<(
        crate::type_aliases::type_or_pack::TypeOrPack,
        crate::type_aliases::type_function_kind::TypeFunctionKind,
    )>,
    pub(crate) types: crate::type_aliases::seen_types_type_function_runtime_builder::SeenTypes,
    pub(crate) packs:
        crate::type_aliases::seen_type_packs_type_function_runtime_builder::SeenTypePacks,
    pub(crate) steps: i32,
}
