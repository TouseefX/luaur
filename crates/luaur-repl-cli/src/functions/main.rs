use crate::functions::repl_main::repl_main;
use core::ffi::c_char;
use luaur_cli_lib::functions::set_luau_flags_default::set_luau_flags_default;
use std::env;
use std::ffi::CString;

pub fn main() {
    set_luau_flags_default();

    let args: Vec<String> = env::args().collect();
    let mut c_args: Vec<CString> = args
        .iter()
        .map(|arg| CString::new(arg.as_str()).unwrap())
        .collect();
    let mut argv: Vec<*mut c_char> = c_args
        .iter_mut()
        .map(|arg| arg.as_ptr() as *mut c_char)
        .collect();

    let exit_code = repl_main(args.len() as i32, argv.as_mut_ptr());
    std::process::exit(exit_code);
}
