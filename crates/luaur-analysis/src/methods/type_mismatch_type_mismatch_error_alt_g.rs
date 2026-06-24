//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Method:Luau.Analysis:Analysis/src/Error.cpp:1062:type_mismatch_type_mismatch`
//! Source: `Analysis/src/Error.cpp`
//! Graph edges:
//! - declared_by: source_file Analysis/src/Error.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Analysis/include/Luau/Clone.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/FileResolver.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file Analysis/include/Luau/ToString.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypeChecker2.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//! - incoming:
//!   - declares <- source_file Analysis/src/Error.cpp
//! - outgoing:
//!   - type_ref -> record TypeMismatch (Analysis/include/Luau/Error.h)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record TypeError (Analysis/include/Luau/Error.h)
//!   - translates_to -> rust_item TypeMismatch::TypeMismatch

use crate::enums::context_error::Context;
use crate::records::type_error::TypeError;
use crate::records::type_mismatch::TypeMismatch;
use crate::type_aliases::type_id::TypeId;

impl TypeMismatch {
    // C++ `TypeMismatch::TypeMismatch(TypeId wantedType, TypeId givenType, std::string reason, std::optional<TypeError> error, TypeMismatch::Context context)`
    // (Error.cpp:1062): wraps the optional error into a shared pointer (`Arc`) when present.
    pub fn from_wanted_given_reason_error_context(
        wanted_type: TypeId,
        given_type: TypeId,
        reason: alloc::string::String,
        error: Option<TypeError>,
        context: Context,
    ) -> Self {
        Self {
            wanted_type,
            given_type,
            context,
            reason,
            error: error.map(alloc::sync::Arc::new),
        }
    }
}
