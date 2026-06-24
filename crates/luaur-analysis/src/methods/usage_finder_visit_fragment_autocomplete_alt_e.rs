use crate::records::usage_finder::UsageFinder;
use luaur_ast::records::ast_type_reference::AstTypeReference;

impl UsageFinder {
    pub fn visit_ast_type_reference(&mut self, ref_: *mut AstTypeReference) -> bool {
        let ref_ = unsafe { &*ref_ };
        if let Some(prefix) = ref_.prefix {
            let prefix_value = unsafe {
                core::ffi::CStr::from_ptr(prefix.value)
                    .to_string_lossy()
                    .into_owned()
            };
            let name_value = unsafe {
                core::ffi::CStr::from_ptr(ref_.name.value)
                    .to_string_lossy()
                    .into_owned()
            };
            self.referenced_imported_bindings
                .push((prefix_value, name_value));
        } else {
            let name_value = unsafe {
                core::ffi::CStr::from_ptr(ref_.name.value)
                    .to_string_lossy()
                    .into_owned()
            };
            self.referenced_bindings.push(name_value);
        }
        true
    }
}
