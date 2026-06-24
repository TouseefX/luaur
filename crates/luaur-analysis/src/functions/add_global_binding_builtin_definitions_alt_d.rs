use crate::records::binding::Binding;
use crate::records::global_types::GlobalTypes;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::ffi::CString;
use alloc::sync::Arc;
use luaur_ast::records::ast_name_table::AstNameTable;

pub fn add_global_binding_builtin_definitions_alt_d(
    globals: &mut GlobalTypes,
    scope: &ScopePtr,
    name: &str,
    binding: Binding,
) {
    let name_cstr = CString::new(name).unwrap();
    let ast_name = unsafe {
        (*(Arc::as_ptr(&globals.global_names.names) as *mut AstNameTable))
            .get_or_add(name_cstr.as_ptr(), name_cstr.as_bytes().len())
    };

    let scope_ptr = Arc::as_ptr(scope) as *mut Scope;
    unsafe {
        (*scope_ptr)
            .bindings
            .insert(Symbol::from_global(ast_name), binding);
    }
}
