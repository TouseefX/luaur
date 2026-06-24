use crate::functions::linear_search_for_binding::linear_search_for_binding;
use alloc::string::String;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::type_aliases::type_id::TypeId;

pub fn lookup_name(scope: *mut Scope, name: &String) -> Option<TypeId> {
    let binding = linear_search_for_binding(scope, name.as_str());
    if let Some(binding_type_id) = binding {
        Some(binding_type_id)
    } else {
        None
    }
}
