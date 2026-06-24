use alloc::string::String;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::type_aliases::type_id::TypeId;

pub fn linear_search_for_binding(scope: *mut Scope, name: &str) -> Option<TypeId> {
    if scope.is_null() {
        return None;
    }

    unsafe {
        (*scope)
            .linear_search_for_binding(&String::from(name), true)
            .map(|binding| binding.type_id)
    }
}
