use crate::records::binding::Binding;
use crate::records::global_types::GlobalTypes;
use crate::records::symbol::Symbol;
use std::ffi::CString;

pub fn try_get_global_binding_ref(globals: &mut GlobalTypes, name: &str) -> *mut Binding {
    let name = match CString::new(name) {
        Ok(name) => name,
        Err(_) => return core::ptr::null_mut(),
    };

    let ast_name = globals.global_names.names.get(name.as_ptr());
    if ast_name.value.is_null() {
        return core::ptr::null_mut();
    }

    if let Some(binding) = globals
        .global_scope
        .bindings
        .get(&Symbol::from_global(ast_name))
    {
        binding as *const Binding as *mut Binding
    } else {
        core::ptr::null_mut()
    }
}
