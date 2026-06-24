//! Node: `cxx:Function:Luau.Bytecode.CLI:CLI/src/Bytecode.cpp:47:parse_args`
//! Source: `CLI/src/Bytecode.cpp`

use alloc::string::String;
use std::ffi::CStr;

use crate::functions::display_help::display_help;

extern "C" {
    fn setLuauFlags(list: *const i8);
}

pub(crate) fn parse_args(argc: i32, argv: *mut *mut i8, summary_file: &mut String) -> bool {
    for i in 1..argc {
        let arg = unsafe { CStr::from_ptr(*argv.add(i as usize)) };
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
            // TODO: set global optimization level
        } else if arg_str.starts_with("-g") {
            let level_str = &arg_str[2..];
            let level: i32 = level_str.parse().unwrap_or(0);
            if level < 0 || level > 2 {
                eprintln!("Error: Debug level must be between 0 and 2 inclusive.");
                return false;
            }
            // globalOptions.debugLevel = level;
            // TODO: set global debug level
        } else if arg_str.starts_with("--summary-file=") {
            let filename = &arg_str[15..];
            if filename.is_empty() {
                eprintln!("Error: filename missing for '--summary-file'.\n");
                return false;
            }
            *summary_file = filename.to_string();
        } else if arg_str.starts_with("--fflags=") {
            let flags = &arg_str[9..];
            let flags_c = std::ffi::CString::new(flags).unwrap();
            unsafe {
                setLuauFlags(flags_c.as_ptr());
            }
        } else if arg_str.starts_with('-') {
            eprintln!("Error: Unrecognized option '{}'.\n", arg_str);
            let argv0 = unsafe { CStr::from_ptr(*argv) };
            display_help(argv0.to_str().unwrap());
        }
    }

    true
}
