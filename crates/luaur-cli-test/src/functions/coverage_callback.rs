use core::ffi::{c_char, c_int};
use std::fs::File;
use std::io::Write;

pub fn coverage_callback(
    context: *mut core::ffi::c_void,
    function: *const c_char,
    linedefined: c_int,
    depth: c_int,
    hits: *const c_int,
    size: usize,
) {
    let f = unsafe { &mut *(context as *mut File) };

    let name = if depth == 0 {
        "<main>".to_string()
    } else if !function.is_null() {
        let func_str = unsafe { core::ffi::CStr::from_ptr(function).to_string_lossy() };
        format!("{}:{}", func_str, linedefined)
    } else {
        format!("<anonymous>:{}", linedefined)
    };

    let _ = writeln!(f, "FN:{},{}", linedefined, name);

    let mut first_hit = true;
    for i in 0..size as usize {
        let hit = unsafe { *hits.add(i) };
        if hit != -1 {
            if first_hit {
                let _ = writeln!(f, "FNDA:{},{}", hit, name);
                first_hit = false;
            }
            let _ = writeln!(f, "DA:{},{}", i, hit);
        }
    }
}
