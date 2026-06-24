//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:1106-1109`
//!
//! ```cpp
//! TypeFunctionTypeId serialize(TypeId ty, TypeFunctionRuntimeBuilderState* state)
//! {
//!     return TypeFunctionSerializer(state).serialize(ty);
//! }
//! ```
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::records::type_function_serializer::TypeFunctionSerializer;
use crate::type_aliases::seen_type_packs_type_function_runtime_builder::SeenTypePacks;
use crate::type_aliases::seen_types_type_function_runtime_builder::SeenTypes;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;

pub fn serialize_type_id_type_function_runtime_builder_state(
    ty: TypeId,
    state: *mut TypeFunctionRuntimeBuilderState,
) -> TypeFunctionTypeId {
    // C++ constructs a temporary `TypeFunctionSerializer(state)` and immediately calls
    // `.serialize(ty)`. The `TypeFunctionSerializer(state)` constructor is `type_function_serializer`.
    let mut serializer = TypeFunctionSerializer {
        state: core::ptr::null_mut(),
        type_function_runtime: core::ptr::null_mut(),
        queue: alloc::vec::Vec::new(),
        types: SeenTypes::new(core::ptr::null()),
        packs: SeenTypePacks::new(core::ptr::null()),
        steps: 0,
    };
    serializer.type_function_serializer(state);
    serializer.serialize_type_id(ty)
}
