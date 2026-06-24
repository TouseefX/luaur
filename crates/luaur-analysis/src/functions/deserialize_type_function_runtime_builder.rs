use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::type_aliases::type_function_type_id::TypeFunctionTypeId;
use crate::type_aliases::type_id::TypeId;

pub fn deserialize_type_function_type_id_type_function_runtime_builder_state(
    ty: TypeFunctionTypeId,
    state: *mut TypeFunctionRuntimeBuilderState,
) -> TypeId {
    let mut deserializer = TypeFunctionDeserializer::default();
    deserializer.type_function_deserializer(state);
    deserializer.deserialize_type_function_type_id(ty)
}
