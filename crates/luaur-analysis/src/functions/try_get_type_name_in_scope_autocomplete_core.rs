use crate::functions::can_suggest_inferred_type_autocomplete_core::can_suggest_inferred_type;
use crate::functions::try_to_string_detailed::try_to_string_detailed;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;

pub fn try_get_type_name_in_scope(
    scope: ScopePtr,
    ty: TypeId,
    function_type_arguments: bool,
) -> Option<Name> {
    if !can_suggest_inferred_type(ty) {
        return None;
    }

    try_to_string_detailed(scope, ty, function_type_arguments)
}
