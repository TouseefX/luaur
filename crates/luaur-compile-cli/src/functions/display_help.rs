use core::ffi::c_char;

#[allow(non_snake_case)]
pub fn display_help(argv0: *const c_char) {
    unsafe {
        // Use std::print! to avoid dependency on libc crate, matching the behavior of the C++ printf call
        let argv0_str = core::ffi::CStr::from_ptr(argv0).to_string_lossy();
        std::print!("Usage: {} [--mode] [options] [file list]\n", argv0_str);
        std::print!("\n");
        std::print!("Available modes:\n");
        std::print!("   binary, text, remarks, codegen, codegenir, codegenasm, codegenverbose, codegennull, null\n");
        std::print!("\n");
        std::print!("Available options:\n");
        std::print!("  -h, --help: Display this usage message.\n");
        std::print!("  -O<n>: compile with optimization level n (default 1, n should be between 0 and 2).\n");
        std::print!(
            "  -g<n>: compile with debug level n (default 1, n should be between 0 and 2).\n"
        );
        std::print!("  --target=<target>: compile code for specific architecture (a64, x64, a64_nf, x64_ms).\n");
        std::print!("  --timetrace: record compiler time tracing information into trace.json\n");
        std::print!("  --record-stats=<granularity>: granularity of compilation stats (total, file, function).\n");
        std::print!("  --bytecode-summary: Compute bytecode operation distribution.\n");
        std::print!(
            "  --dump-constants: Dump constant table for each function (text mode only).\n"
        );
        std::print!("  --stats-file=<filename>: file in which compilation stats will be recored (default 'stats.json').\n");
        std::print!(
            "  --vector-lib=<name>: name of the library providing vector type operations.\n"
        );
        std::print!("  --vector-ctor=<name>: name of the function constructing a vector value.\n");
        std::print!("  --vector-type=<name>: name of the vector type.\n");
        std::print!("  --only-parse: Only parse the input.\n");
        std::print!("  --parse-cst: Whether parser should parse CST in addition to AST.\n");
        std::print!("  --fflags=<flags>: comma-separated list of fast flags to enable/disable (--fflags=true,false,LuauFlag1=true,LuauFlag2=false).\n");
    }
}
