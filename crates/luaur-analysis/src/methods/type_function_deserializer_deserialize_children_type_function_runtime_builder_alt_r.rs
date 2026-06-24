//! Source: `Analysis/src/TypeFunctionRuntimeBuilder.cpp:1095-1098`
//!
//! ```cpp
//! void deserializeChildren(TypeFunctionVariadicTypePack* v2, VariadicTypePack* v1)
//! {
//!     v1->ty = shallowDeserialize(v2->type);
//! }
//! ```
use crate::records::type_function_deserializer::TypeFunctionDeserializer;
use crate::records::type_function_variadic_type_pack::TypeFunctionVariadicTypePack;
use crate::records::variadic_type_pack::VariadicTypePack;

impl TypeFunctionDeserializer {
    pub fn deserialize_children_type_function_variadic_type_pack_variadic_type_pack(
        &mut self,
        v2: *mut TypeFunctionVariadicTypePack,
        v1: *mut VariadicTypePack,
    ) {
        unsafe {
            let deserialized = self.shallow_deserialize_type_function_type_id((*v2).type_id);
            (*v1).ty = deserialized;
        }
    }
}
