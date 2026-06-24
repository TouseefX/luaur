use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::type_aliases::type_function_type_pack_id::TypeFunctionTypePackId;
use crate::type_aliases::type_pack_id::TypePackId;

impl TypeFunctionDeserializer {
    pub fn find_type_function_type_pack_id(
        &self,
        tp: TypeFunctionTypePackId,
    ) -> Option<TypePackId> {
        self.packs.get(&tp).copied()
    }
}
