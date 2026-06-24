use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_runtime_builder_state::TypeFunctionRuntimeBuilderState;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn deserialize_type_function_type_pack_id_type_function_runtime_builder_state(
    tp: TypeFunctionTypePackId,
    state: *mut TypeFunctionRuntimeBuilderState,
) -> TypePackId {
    let mut deserializer = TypeFunctionDeserializer::default();
    deserializer.type_function_deserializer(state);
    deserializer.deserialize_type_function_type_pack_id(tp)
}
