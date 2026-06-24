//! Source: `Analysis/src/OverloadResolver.cpp:645-658` (hand-ported)
use crate::records::overload_resolver::OverloadResolver;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl OverloadResolver {
    pub fn maybe_emplace_error_error_vec_location_subtyping_reasoning_optional_type_id_optional_type_id(
        &self,
        errors: *mut ErrorVec,
        arg_location: Location,
        reason: *const SubtypingReasoning,
        wanted_type: Option<TypeId>,
        given_type: Option<TypeId>,
    ) {
        // This is a temporary compatibility shim for the old API. It's ok to pass
        // an empty ModuleName here because the caller of
        // OverloadResolver::resolve() will overwrite the moduleName of any errors
        // that are reported.
        let module_name: ModuleName = ModuleName::new();
        self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_id_optional_type_id(
            errors,
            arg_location,
            &module_name,
            reason,
            wanted_type,
            given_type,
        )
    }
}
