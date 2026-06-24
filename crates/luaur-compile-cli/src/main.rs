//! `luaur-compile` — command-line Luau source-to-bytecode compiler (binary entry point).
//!
//! Thin wrapper over the library `main(argc, argv)` (faithful port of the upstream
//! `luau-compile` CLI in CLI/src/Compile.cpp). Marshals `std::env::args()` into a
//! NUL-terminated owned `argv` so the FileUtils/Flags ports (which take
//! `int argc, char** argv`) can be called faithfully.

use core::ffi::c_char;
use std::ffi::CString;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut c_args: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = c_args
        .iter_mut()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();

    let exit_code = luaur_compile_cli::functions::main::main(args.len() as i32, argv.as_mut_ptr());
    std::process::exit(exit_code);
}
