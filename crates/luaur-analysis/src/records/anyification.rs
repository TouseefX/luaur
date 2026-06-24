use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::scope::Scope;
use crate::records::substitution::Substitution;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

#[derive(Debug, Clone)]
pub struct Anyification {
    pub(crate) base: Substitution,
    pub(crate) scope: *mut Scope,
    pub(crate) builtin_types: *const BuiltinTypes,
    pub(crate) ice_handler: *mut InternalErrorReporter,
    pub(crate) any_type: TypeId,
    pub(crate) any_type_pack: TypePackId,
    pub normalization_too_complex: bool,
}
