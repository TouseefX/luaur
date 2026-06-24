use crate::records::builtin_types::BuiltinTypes;
use crate::type_aliases::type_id::TypeId;

impl BuiltinTypes {
    pub fn error_recovery_type(&self, guess: TypeId) -> TypeId {
        guess
    }
}
