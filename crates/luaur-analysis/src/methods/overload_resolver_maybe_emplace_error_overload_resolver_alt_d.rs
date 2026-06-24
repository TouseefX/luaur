//! Source: `Analysis/src/OverloadResolver.cpp:722-744` (hand-ported)
use crate::records::overload_resolver::OverloadResolver;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_name_type_fwd::ModuleName;
use crate::type_aliases::type_or_pack::TypeOrPack;
use luaur_ast::records::location::Location;

impl OverloadResolver {
    pub fn maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_or_pack_optional_type_or_pack(
        &self,
        errors: *mut ErrorVec,
        arg_location: Location,
        module_name: &ModuleName,
        reason: *const SubtypingReasoning,
        wanted_type: Option<TypeOrPack>,
        given_type: Option<TypeOrPack>,
    ) {
        if wanted_type.is_none() || given_type.is_none() {
            return;
        }

        let wanted_type = wanted_type.unwrap();
        let given_type = given_type.unwrap();

        let wanted_ty = wanted_type.get_if_0();
        let given_ty = given_type.get_if_0();
        if let (Some(&wanted_ty), Some(&given_ty)) = (wanted_ty, given_ty) {
            return self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_id_optional_type_id(
                errors,
                arg_location,
                module_name,
                reason,
                Some(wanted_ty),
                Some(given_ty),
            );
        }

        let wanted_tp = wanted_type.get_if_1();
        let given_tp = given_type.get_if_1();

        if let (Some(&wanted_tp), Some(&given_tp)) = (wanted_tp, given_tp) {
            return self.maybe_emplace_error_error_vec_location_module_name_subtyping_reasoning_optional_type_pack_id_optional_type_pack_id(
                errors,
                arg_location,
                module_name,
                reason,
                Some(wanted_tp),
                Some(given_tp),
            );
        }
    }
}
