//! Source: `Analysis/src/Normalize.cpp:3749-3762` (hand-ported)
//!
//! Free function `bool isSubtype(TypeId, TypeId, NotNull<TypeArena>,
//! NotNull<BuiltinTypes>, NotNull<Scope>, NotNull<Normalizer>,
//! NotNull<TypeFunctionRuntime>, NotNull<InternalErrorReporter>)`.
use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::type_id::TypeId;

pub fn is_subtype(
    sub_ty: TypeId,
    super_ty: TypeId,
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    scope: *mut Scope,
    normalizer: *mut Normalizer,
    type_function_runtime: *mut TypeFunctionRuntime,
    reporter: *mut InternalErrorReporter,
) -> bool {
    let mut subtyping = Subtyping::subtyping_owned(
        builtin_types,
        arena,
        normalizer,
        type_function_runtime,
        reporter,
    );
    subtyping
        .is_subtype_type_id_type_id_not_null_scope(sub_ty, super_ty, scope)
        .is_subtype
}
