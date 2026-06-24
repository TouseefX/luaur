use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::{c_char, c_void};
use std::ptr;

use luaur_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use crate::functions::serialize_script_summary::serialize_script_summary;

pub fn serialize_summaries(
    files: &Vec<String>,
    script_summaries: &Vec<Vec<FunctionBytecodeSummary>>,
    summary_file: &String,
) -> bool {
    let fp = unsafe {
        fopen(
            summary_file.as_ptr() as *const c_char,
            b"w\0".as_ptr() as *const c_char,
        )
    };
    let file_count = files.len();

    if fp.is_null() {
        unsafe {
            fprintf(
                stderr,
                b"Unable to open '%s'.\n\0".as_ptr() as *const c_char,
                summary_file.as_ptr() as *const c_char,
            );
        }
        return false;
    }

    unsafe {
        fprintf(fp, b"{\n\0".as_ptr() as *const c_char);
    }

    for i in 0..file_count {
        serialize_script_summary(&files[i], &script_summaries[i], fp);
        unsafe {
            if i < file_count - 1 {
                fprintf(fp, b",\n\0".as_ptr() as *const c_char);
            } else {
                fprintf(fp, b"\n\0".as_ptr() as *const c_char);
            }
        }
    }

    unsafe {
        fprintf(fp, b"}\0".as_ptr() as *const c_char);
        fclose(fp);
    }

    true
}

extern "C" {
    static mut stderr: *mut c_void;
    fn fopen(filename: *const c_char, mode: *const c_char) -> *mut c_void;
    fn fclose(stream: *mut c_void) -> i32;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> i32;
}
