use crate::functions::can_suggest_inferred_type_autocomplete_core_alt_b::can_suggest_inferred_type_type_pack_id;
use crate::functions::try_to_string_detailed::try_to_string_detailed;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_pack_id::TypePackId;

pub fn try_get_type_name_in_scope_scope_ptr_type_pack_id_bool(
    scope: ScopePtr,
    tp: TypePackId,
    function_type_arguments: bool,
) -> Option<Name> {
    if !can_suggest_inferred_type_type_pack_id(tp) {
        return None;
    }

    try_to_string_detailed(scope, tp, function_type_arguments)
}
