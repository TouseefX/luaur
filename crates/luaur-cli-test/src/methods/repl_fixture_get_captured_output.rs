use crate::records::repl_fixture::ReplFixture;

use alloc::string::String;

use core::ffi::c_char;

use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::macros::lua_getglobal::lua_getglobal;
use luaur_vm::macros::lua_pop::lua_pop;

impl ReplFixture {
    pub fn get_captured_output(&mut self) -> String {
        unsafe {
            lua_getglobal(
                self.l as *mut _,
                c"capturedoutput".as_ptr() as *const c_char,
            );
            let str_ptr = lua_tolstring(self.l as *mut _, -1, core::ptr::null_mut());
            let result = if str_ptr.is_null() {
                String::new()
            } else {
                let cs = core::ffi::CStr::from_ptr(str_ptr);
                cs.to_string_lossy().into_owned()
            };
            lua_pop(self.l as *mut _, 1);
            result
        }
    }
}
