use crate::functions::flatten_type_pack::flatten_type_pack_id;
use crate::functions::follow_type::follow_type_id;
use crate::functions::is_optional_type::is_optional_type;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::overload_resolver::OverloadResolver;
use crate::type_aliases::type_pack_id::TypePackId;

impl OverloadResolver {
    pub fn is_arity_compatible(
        &self,
        candidate: TypePackId,
        desired: TypePackId,
        builtin_types: *mut BuiltinTypes,
    ) -> bool {
        let (candidate_head, candidate_tail) = flatten_type_pack_id(candidate);
        let (desired_head, desired_tail) = flatten_type_pack_id(desired);

        if candidate_head.len() < desired_head.len() {
            if candidate_tail.is_some() {
                return true;
            }

            for i in candidate_head.len()..desired_head.len() {
                let ty = unsafe { follow_type_id(desired_head[i]) };
                if !is_optional_type(ty, builtin_types) {
                    return false;
                }
            }
        }

        if candidate_head.len() > desired_head.len() {
            return desired_tail.is_some();
        }

        true
    }
}
