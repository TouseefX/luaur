use crate::enums::type_type_function_runtime::Type;
use crate::records::type_function_primitive_type::TypeFunctionPrimitiveType;

impl TypeFunctionPrimitiveType {
    pub fn new(r#type: Type) -> Self {
        Self { r#type }
    }
}
