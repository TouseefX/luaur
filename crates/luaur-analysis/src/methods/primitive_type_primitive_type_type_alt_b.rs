use crate::records::primitive_type::{PrimitiveType, Type};
use crate::type_aliases::type_id::TypeId;

impl PrimitiveType {
    pub fn primitive_type_type_item_type_id(r#type: Type, metatable: TypeId) -> Self {
        Self {
            r#type,
            metatable: Some(metatable),
        }
    }
}
