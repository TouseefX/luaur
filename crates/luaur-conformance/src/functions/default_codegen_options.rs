use luaur_code_gen::enums::code_gen_flags::CodeGenFlags;
use luaur_code_gen::records::compilation_options::CompilationOptions;

pub fn default_codegen_options() -> CompilationOptions {
    let mut opts = CompilationOptions {
        flags: 0,
        hooks: unsafe { core::mem::zeroed() },
        userdata_types: core::ptr::null(),
        record_counters: false,
        nop_padding: false,
    };

    opts.flags = CodeGenFlags::CodeGen_ColdFunctions as u32;
    opts
}
