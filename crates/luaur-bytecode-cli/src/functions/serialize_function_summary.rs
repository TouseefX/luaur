use core::ffi::c_void;
use std::ffi::CString;

use luaur_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

/// Source: `CLI/src/Bytecode.cpp:185-215` (`serializeFunctionSummary`).
///
/// The `%s` fields take C-string pointers. `getSource()`/`getName()` return a
/// non-NUL-terminated `&str`, so they are copied into NUL-terminated `CString`s
/// (the C++ uses `std::string::c_str()`) before being handed to `fprintf`.
pub fn serialize_function_summary(summary: &FunctionBytecodeSummary, fp: *mut c_void) {
    let nesting_limit = summary.get_nesting_limit();
    let op_limit = summary.get_op_limit();

    let source = CString::new(summary.get_source().replace('\0', "")).unwrap();
    let name = CString::new(summary.get_name().replace('\0', "")).unwrap();

    unsafe {
        // Write opening brace
        fprintf(fp, c"        {\n".as_ptr());
        // Write source
        fprintf(
            fp,
            c"            \"source\": \"%s\",\n".as_ptr(),
            source.as_ptr(),
        );
        // Write name
        fprintf(
            fp,
            c"            \"name\": \"%s\",\n".as_ptr(),
            name.as_ptr(),
        );
        // Write line
        fprintf(
            fp,
            c"            \"line\": %d,\n".as_ptr(),
            summary.get_line(),
        );

        // Write nestingLimit
        fprintf(
            fp,
            c"            \"nestingLimit\": %u,\n".as_ptr(),
            nesting_limit,
        );

        // Write counts array header
        fprintf(fp, c"            \"counts\": [".as_ptr());

        // Iterate over nesting levels
        for nesting in 0..=nesting_limit {
            fprintf(fp, c"\n                [".as_ptr());

            // Iterate over opcodes
            for i in 0..op_limit {
                let count = summary.get_count(nesting, i as u8);
                fprintf(fp, c"%d".as_ptr(), count);

                if i < op_limit - 1 {
                    fprintf(fp, c", ".as_ptr());
                }
            }

            fprintf(fp, c"]".as_ptr());

            if nesting < nesting_limit {
                fprintf(fp, c",".as_ptr());
            }
        }

        // Close counts array and object
        fprintf(fp, c"\n            ]".as_ptr());
        fprintf(fp, c"\n        }".as_ptr());
    }
}

extern "C" {
    fn fprintf(stream: *mut c_void, format: *const core::ffi::c_char, ...) -> core::ffi::c_int;
}
