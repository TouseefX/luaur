//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:1111-1114`
//!
//! ```cpp
//! TypeFunctionTypePackId serialize(TypePackId tp, TypeFunctionRuntimeBuilderState* state)
//! {
//!     return TypeFunctionSerializer(state).serialize(tp);
//! }
//! ```
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::type_aliases::seen_type_packs_type_function_runtime_builder::SeenTypePacks;
use crate::type_aliases::seen_types_type_function_runtime_builder::SeenTypes;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn serialize_type_pack_id_type_function_runtime_builder_state(
    tp: TypePackId,
    state: *mut TypeFunctionRuntimeBuilderState,
) -> TypeFunctionTypePackId {
    let mut serializer = TypeFunctionSerializer {
        state: core::ptr::null_mut(),
        type_function_runtime: core::ptr::null_mut(),
        queue: alloc::vec::Vec::new(),
        types: SeenTypes::new(core::ptr::null()),
        packs: SeenTypePacks::new(core::ptr::null()),
        steps: 0,
    };
    serializer.type_function_serializer(state);
    serializer.serialize_type_pack_id(tp)
}
