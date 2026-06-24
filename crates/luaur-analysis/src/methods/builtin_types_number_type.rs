use crate::records::builtin_types::BuiltinTypes;
use crate::type_aliases::type_id::TypeId;

impl BuiltinTypes {
    pub fn number_type(&self) -> TypeId {
        self.numberType
    }
}
