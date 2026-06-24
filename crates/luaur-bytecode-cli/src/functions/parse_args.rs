//! Node: `cxx:Function:Luau.Bytecode.CLI:CLI/src/Bytecode.cpp:47:parse_args`
//! Source: `CLI/src/Bytecode.cpp:47-97`

use alloc::string::String;
use core::ffi::c_char;
use std::ffi::CStr;

use luaur_cli_lib::functions::set_luau_flags_flags_alt_b::set_luau_flags_c_char;

use crate::functions::display_help::display_help;
use crate::records::global_options::globalOptions;

pub(crate) fn parse_args(argc: i32, argv: *mut *mut i8, summary_file: &mut String) -> bool {
    for i in 1..argc {
        let arg_ptr = unsafe { *argv.add(i as usize) };
        let arg = unsafe { CStr::from_ptr(arg_ptr) };
        let arg_str = arg.to_str().unwrap();

        if arg_str == "-h" || arg_str == "--help" {
            let argv0 = unsafe { CStr::from_ptr(*argv) };
            display_help(argv0.to_str().unwrap());
        } else if arg_str.starts_with("-O") {
            let level_str = &arg_str[2..];
            let level: i32 = level_str.parse().unwrap_or(0);
            if level < 0 || level > 2 {
                eprintln!("Error: Optimization level must be between 0 and 2 inclusive.");
                return false;
            }
            // globalOptions.optimizationLevel = level;
            unsafe {
                globalOptions.optimization_level = level;
            }
        } else if arg_str.starts_with("-g") {
            let level_str = &arg_str[2..];
            let level: i32 = level_str.parse().unwrap_or(0);
            if level < 0 || level > 2 {
                eprintln!("Error: Debug level must be between 0 and 2 inclusive.");
                return false;
            }
            // globalOptions.debugLevel = level;
            unsafe {
                globalOptions.debug_level = level;
            }
        } else if arg_str.starts_with("--summary-file=") {
            let filename = &arg_str[15..];
            if filename.is_empty() {
                eprintln!("Error: filename missing for '--summary-file'.\n");
                return false;
            }
            *summary_file = filename.to_string();
        } else if arg_str.starts_with("--fflags=") {
            // setLuauFlags(argv[i] + 9);
            unsafe {
                set_luau_flags_c_char(arg_ptr.add(9) as *const c_char);
            }
        } else if arg_str.starts_with('-') {
            eprintln!("Error: Unrecognized option '{}'.\n", arg_str);
            let argv0 = unsafe { CStr::from_ptr(*argv) };
            display_help(argv0.to_str().unwrap());
        }
    }

    true
}
