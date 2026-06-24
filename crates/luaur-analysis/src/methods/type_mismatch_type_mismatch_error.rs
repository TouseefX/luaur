use crate::records::type_mismatch::TypeMismatch;

impl TypeMismatch {
    pub fn type_mismatch() -> Self {
        Self {
            wanted_type: core::ptr::null(),
            given_type: core::ptr::null(),
            reason: alloc::string::String::new(),
            error: None,
            context: crate::enums::context_error::Context::CovariantContext,
        }
    }

    // C++ `TypeMismatch::TypeMismatch(TypeId wantedType, TypeId givenType)`
    // (Error.cpp:1026): sets the two types and defaults reason/error/context.
    pub fn from_wanted_given(
        wanted_type: crate::type_aliases::type_id::TypeId,
        given_type: crate::type_aliases::type_id::TypeId,
    ) -> Self {
        Self {
            wanted_type,
            given_type,
            reason: alloc::string::String::new(),
            error: None,
            context: crate::enums::context_error::Context::CovariantContext,
        }
    }
}
