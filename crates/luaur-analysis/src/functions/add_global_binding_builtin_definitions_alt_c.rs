use crate::functions::add_global_binding_builtin_definitions_alt_d::add_global_binding_builtin_definitions_alt_d;
use crate::records::binding::Binding;
use crate::records::global_types::GlobalTypes;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::format;
use alloc::string::String;
use luaur_ast::records::location::Location;

pub fn add_global_binding_builtin_definitions_alt_c(
    globals: &mut GlobalTypes,
    scope: &ScopePtr,
    name: &str,
    ty: TypeId,
    package_name: &str,
) {
    let documentation_symbol: String = format!("{}/global/{}", package_name, name);
    add_global_binding_builtin_definitions_alt_d(
        globals,
        scope,
        name,
        Binding {
            type_id: ty,
            location: Location::default(),
            deprecated: false,
            deprecated_suggestion: String::new(),
            documentation_symbol: Some(documentation_symbol),
        },
    );
}
