use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::type_aliases::type_function_kind::TypeFunctionKind;
use crate::type_aliases::type_or_pack::TypeOrPack;

impl TypeFunctionDeserializer {
    pub fn find_type_function_kind(&self, kind: TypeFunctionKind) -> Option<TypeOrPack> {
        match kind {
            TypeFunctionKind::V0(ty) => self.find_type_function_type_id(ty).map(TypeOrPack::V0),
            TypeFunctionKind::V1(tp) => {
                self.find_type_function_type_pack_id(tp).map(TypeOrPack::V1)
            }
        }
    }
}
