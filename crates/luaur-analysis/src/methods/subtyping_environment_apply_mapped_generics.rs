use crate::records::apply_mapped_generics::ApplyMappedGenerics;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::substitution::Substitution;
use crate::records::subtyping_environment::SubtypingEnvironment;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;

impl SubtypingEnvironment {
    /// C++ `SubtypingEnvironment::applyMappedGenerics` (`Subtyping.cpp:504-513`):
    ///
    /// ```cpp
    /// ApplyMappedGenerics amg{builtinTypes, arena, *this, iceReporter};
    /// return amg.substitute(ty);
    /// ```
    ///
    /// `ApplyMappedGenerics` extends `Substitution` and inherits `substitute`,
    /// whose traversal virtual-dispatches into the overridden `isDirty` /
    /// `clean` / `ignoreChildren`. The Rust `ApplyMappedGenerics` now embeds
    /// `base: Substitution` and installs those overrides into the
    /// `SubstitutionVtable` from its `substitute_type_id` wrapper.
    pub fn apply_mapped_generics(
        &mut self,
        builtin_types: *mut BuiltinTypes,
        arena: *mut TypeArena,
        ty: TypeId,
        ice_reporter: *mut InternalErrorReporter,
    ) -> Option<TypeId> {
        let mut amg = ApplyMappedGenerics {
            base: Substitution::substitution_new(TxnLog::empty(), arena),
            builtin_types,
            arena,
            ice_reporter,
            env: self as *mut SubtypingEnvironment,
        };
        amg.substitute_type_id(ty)
    }
}
