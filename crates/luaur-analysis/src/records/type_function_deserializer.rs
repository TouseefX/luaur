//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:552-581`

use crate::records::serialized_function_scope::SerializedFunctionScope;
use crate::records::serialized_generic::SerializedGeneric;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::type_aliases::seen_type_packs_type_function_runtime_builder_alt_d::SeenTypePacks;
use crate::type_aliases::seen_types_type_function_runtime_builder_alt_d::SeenTypes;
use crate::type_aliases::type_function_kind::TypeFunctionKind;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::TypeOrPack;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

#[derive(Debug, Clone)]
pub struct TypeFunctionDeserializer {
    pub(crate) state: *mut TypeFunctionRuntimeBuilderState,
    pub(crate) type_function_runtime: *mut TypeFunctionRuntime,
    pub(crate) queue: Vec<(TypeFunctionKind, TypeOrPack)>,
    pub(crate) generic_types: Vec<SerializedGeneric<TypeId>>,
    pub(crate) generic_packs: Vec<SerializedGeneric<TypePackId>>,
    pub(crate) function_scopes: Vec<SerializedFunctionScope>,
    pub(crate) types: SeenTypes,
    pub(crate) packs: SeenTypePacks,
    pub(crate) steps: i32,
}

impl Default for TypeFunctionDeserializer {
    fn default() -> Self {
        Self {
            state: core::ptr::null_mut(),
            type_function_runtime: core::ptr::null_mut(),
            queue: Vec::new(),
            generic_types: Vec::new(),
            generic_packs: Vec::new(),
            function_scopes: Vec::new(),
            types: SeenTypes::new(core::ptr::null()),
            packs: SeenTypePacks::new(core::ptr::null()),
            steps: 0,
        }
    }
}
