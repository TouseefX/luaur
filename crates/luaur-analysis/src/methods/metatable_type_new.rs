use crate::records::metatable_type::MetatableType;
use crate::type_aliases::type_id::TypeId;

impl MetatableType {
    pub fn new(table: TypeId, metatable: TypeId) -> Self {
        Self {
            table,
            metatable,
            syntheticName: None,
        }
    }
}
