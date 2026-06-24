extern crate alloc;

use alloc::string::String;
use luaur_ast::records::ast_local::AstLocal;

pub fn get_local_name(local: *mut AstLocal) -> String {
    unsafe {
        if !local.is_null() && !(*local).name.value.is_null() {
            return core::ffi::CStr::from_ptr((*local).name.value)
                .to_string_lossy()
                .into_owned();
        }
    }
    String::from("?")
}
