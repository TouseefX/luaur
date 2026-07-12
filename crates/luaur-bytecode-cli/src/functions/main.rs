use alloc::string::String;
use alloc::vec::Vec;
use core::ffi::c_char;

use luaur_cli_lib::functions::get_source_files::get_source_files;
use luaur_cli_lib::functions::set_luau_flags_default::set_luau_flags_default;
use luaur_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;
use luaur_common::functions::assert_handler::assert_handler;

use crate::functions::analyze_file::analyze_file;
use crate::functions::assertion_handler::assertion_handler;
use crate::functions::parse_args::parse_args;
use crate::functions::serialize_summaries::serialize_summaries;

unsafe extern "C" fn assertion_handler_adapter(
    expr: *const c_char,
    file: *const c_char,
    line: i32,
    function: *const c_char,
) -> i32 {
    assertion_handler(expr, file, line, function)
}

pub fn main(argc: i32, argv: *mut *mut core::ffi::c_char) -> i32 {
    *assert_handler() = Some(assertion_handler_adapter);

    set_luau_flags_default();

    let mut summary_file = String::from("bytecode-summary.json");
    let nesting_limit = 0;

    if !parse_args(argc, argv, &mut summary_file) {
        return 1;
    }

    let files = get_source_files(argc, argv);
    let file_count = files.len();

    let mut script_summaries: Vec<Vec<FunctionBytecodeSummary>> = Vec::new();
    script_summaries.reserve(file_count);

    for file in &files {
        let mut script_summary = Vec::new();

        if !analyze_file(file, nesting_limit, &mut script_summary) {
            return 1;
        }

        script_summaries.push(script_summary);
    }

    let summary_file_with_nul = String::from(summary_file.clone() + "\0");

    if !serialize_summaries(&files, &script_summaries, &summary_file_with_nul) {
        return 1;
    }

    println!("Bytecode summary written to '{}'", summary_file);

    0
}
