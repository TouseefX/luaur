use crate::records::function_counters::FunctionCounters;
use crate::records::module_counters::ModuleCounters;
use core::ffi::{c_char, c_int, c_void, CStr};

pub fn counters_function_callback(
    context: *mut c_void,
    function: *const c_char,
    line_defined: c_int,
) {
    let counters = unsafe { &mut *(context as *mut ModuleCounters) };

    let name = if function.is_null() && line_defined == 1 {
        "<main>".to_string()
    } else if !function.is_null() {
        let func_str = unsafe { CStr::from_ptr(function).to_string_lossy() };
        format!("{}:{}", func_str, line_defined)
    } else {
        format!("<anonymous>:{}", line_defined)
    };

    let _ = name; // FunctionCounters currently does not expose a public `name` field in the Rust record

    let func = unsafe { core::mem::zeroed::<FunctionCounters>() };
    counters.functions.push(func);
}
