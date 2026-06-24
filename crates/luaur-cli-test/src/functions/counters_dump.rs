use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{c_char, c_int, c_void, CStr};

use crate::records::function_counters::FunctionCounters;
use crate::records::line_counters::LineCounters;
use crate::records::module_counters::ModuleCounters;
use luaur_common::FFlag;
use luaur_common::FInt;

use luaur_vm::functions::lua_getcounters;
use luaur_vm::functions::lua_getinfo;
use luaur_vm::macros::lua_getref::lua_getref;
use luaur_vm::macros::lua_pop::lua_pop;

#[allow(non_upper_case_globals)]
extern "C" {
    fn fopen(path: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(file: *mut c_void) -> c_int;
    fn fprintf(file: *mut c_void, fmt: *const c_char, ...) -> c_int;
    fn printf(fmt: *const c_char, ...) -> c_int;
}

unsafe extern "C" fn counters_function_callback(
    context: *mut c_void,
    function: *const c_char,
    line_defined: c_int,
) {
    let counters = &mut *(context as *mut ModuleCounters);

    let name: String = if function.is_null() && line_defined == 1 {
        "<main>".to_string()
    } else if !function.is_null() {
        let func_str = CStr::from_ptr(function).to_string_lossy();
        format!("{}:{}", func_str, line_defined)
    } else {
        format!("<anonymous>:{}", line_defined)
    };

    let _ = name;

    let func: FunctionCounters = core::mem::zeroed();
    counters.functions.push(func);
}

unsafe extern "C" fn counters_value_callback(
    context: *mut c_void,
    kind: i32,
    line: i32,
    hits: u64,
) {
    let _ = (context, kind, line, hits);
    // FunctionCounters currently doesn't reliably expose the counters map in the Rust record.
}

pub fn counters_dump(path: &str) {
    unsafe {
        // If counters live in some global on the C++ side, the Rust translation would need
        // accessors for it. Those are not available in the current context, so we can't
        // faithfully reproduce the Lua state traversal/counter collection here.
        //
        // Keep behavior as a stub for now.
        let _ = path;
        // Avoid any FFI calls that would require an initialized Lua state.
    }
}
