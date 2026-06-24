use core::ffi::{c_char, c_int, c_void, CStr};
use std::ffi::CString;
use std::io::Write;

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
}

pub unsafe fn coverage_callback(
    context: *mut c_void,
    function: *const c_char,
    linedefined: c_int,
    depth: c_int,
    hits: *const c_int,
    size: usize,
) {
    if context.is_null() {
        return;
    }

    let name = if depth == 0 {
        "<main>".to_string()
    } else if !function.is_null() {
        let func_str = CStr::from_ptr(function).to_string_lossy();
        format!("{}:{}", func_str, linedefined)
    } else {
        format!("<anonymous>:{}", linedefined)
    };

    let name_c = CString::new(name.clone()).unwrap();
    fprintf(
        context,
        "FN:%d,%s\n\0".as_ptr() as *const c_char,
        linedefined,
        name_c.as_ptr(),
    );

    let hits_slice = core::slice::from_raw_parts(hits, size);

    for i in 0..size {
        if hits_slice[i] != -1 {
            let name_c = CString::new(name.clone()).unwrap();
            fprintf(
                context,
                "FNDA:%d,%s\n\0".as_ptr() as *const c_char,
                hits_slice[i],
                name_c.as_ptr(),
            );
            break;
        }
    }

    for i in 0..size {
        if hits_slice[i] != -1 {
            fprintf(
                context,
                "DA:%d,%d\n\0".as_ptr() as *const c_char,
                i as c_int,
                hits_slice[i],
            );
        }
    }
}
