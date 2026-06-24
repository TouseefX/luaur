pub fn help(args: &[&str]) {
    // SAFETY: args[0] is guaranteed to exist because the CLI always provides a program name
    let program_name =
        unsafe { core::ffi::CStr::from_ptr(args[0].as_ptr() as *const core::ffi::c_char) }
            .to_string_lossy();
    println!("Syntax: {} script command \"search text\"", program_name);
    println!("    Within command, use {{}} as a stand-in for the script being reduced");
    std::process::exit(1);
}
