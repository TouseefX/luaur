use alloc::vec::Vec;
use core::ffi::c_void;

use luaur_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::escape_filename::escape_filename;
use crate::functions::serialize_function_summary::serialize_function_summary;

pub fn serialize_script_summary(
    file: &str,
    script_summary: &Vec<FunctionBytecodeSummary>,
    fp: *mut c_void,
) {
    let escaped = escape_filename(file);
    let function_count = script_summary.len();

    unsafe {
        fprintf(
            fp,
            b"    \"%s\": [\n\0".as_ptr() as *const core::ffi::c_char,
            escaped.as_ptr() as *const core::ffi::c_char,
        );
    }

    for (i, summary) in script_summary.iter().enumerate() {
        serialize_function_summary(summary, fp);
        unsafe {
            if i == function_count - 1 {
                fprintf(fp, b"\n\0".as_ptr() as *const core::ffi::c_char);
            } else {
                fprintf(fp, b",\n\0".as_ptr() as *const core::ffi::c_char);
            }
        }
    }

    unsafe {
        fprintf(fp, b"    ]\0".as_ptr() as *const core::ffi::c_char);
    }
}

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
