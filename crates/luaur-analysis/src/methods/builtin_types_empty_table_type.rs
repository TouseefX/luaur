use crate::records::builtin_types::BuiltinTypes;
use crate::type_aliases::type_id::TypeId;

impl BuiltinTypes {
    pub fn empty_table_type(&self) -> TypeId {
        self.emptyTableType
    }
}
