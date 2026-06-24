use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::subtyping_unifier::SubtypingUnifier;
use crate::records::type_arena::TypeArena;

impl SubtypingUnifier {
    pub fn subtyping_unifier(
        arena: *mut TypeArena,
        builtin_types: *mut BuiltinTypes,
        reporter: *mut InternalErrorReporter,
    ) -> Self {
        Self {
            arena,
            builtin_types,
            reporter,
        }
    }
}
