/// C++ `ApplyMappedGenerics` (`Subtyping.cpp`): a `Substitution` subclass, so it
/// embeds `base: Substitution` and inherits `substitute` (whose traversal
/// virtual-dispatches into the `isDirty` / `clean` / `ignoreChildren` overrides
/// installed via [`SubstitutionVtable`](crate::records::tarjan::SubstitutionVtable)).
#[derive(Debug, Clone)]
pub struct ApplyMappedGenerics {
    pub(crate) base: crate::records::substitution::Substitution,
    pub(crate) builtin_types: *mut crate::records::builtin_types::BuiltinTypes,
    pub(crate) arena: *mut crate::records::type_arena::TypeArena,
    pub(crate) ice_reporter: *mut crate::records::internal_error_reporter::InternalErrorReporter,
    pub(crate) env: *mut crate::records::subtyping_environment::SubtypingEnvironment,
}
