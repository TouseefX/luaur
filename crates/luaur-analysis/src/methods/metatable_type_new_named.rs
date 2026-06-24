use crate::records::metatable_type::MetatableType;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;

impl MetatableType {
    pub fn new_named(table: TypeId, metatable: TypeId, synthetic_name: String) -> Self {
        Self {
            table,
            metatable,
            syntheticName: Some(synthetic_name),
        }
    }
}
