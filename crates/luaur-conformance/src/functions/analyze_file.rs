use alloc::string::String;

use luaur_code_gen::functions::summarize_bytecode::summarize_bytecode;
use luaur_code_gen::records::function_bytecode_summary::FunctionBytecodeSummary;

use luaur_ast::records::parse_options::ParseOptions;
use luaur_compiler::functions::compile_or_throw_compiler_alt_b::compile_or_throw_bytecode_builder_string_compile_options_parse_options;
use luaur_compiler::records::compile_options::CompileOptions;

use luaur_bytecode::records::bytecode_builder::BytecodeBuilder;

use luaur_vm::functions::lua_close::lua_close;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::functions::luau_load::luau_load;

use luaur_vm::type_aliases::lua_state::lua_State;

pub fn analyze_file(
    source: &str,
    nesting_limit: u32,
    opt_level: u32,
) -> Vec<FunctionBytecodeSummary> {
    let mut bytecode_builder = BytecodeBuilder::new(None);

    let options = CompileOptions {
        optimization_level: opt_level as core::ffi::c_int,
        debug_level: 1,
        type_info_level: 1,
        coverage_level: 0,
        vector_lib: core::ptr::null(),
        vector_ctor: core::ptr::null(),
        vector_type: core::ptr::null(),
        mutable_globals: core::ptr::null(),
        userdata_types: core::ptr::null(),
        libraries_with_known_members: core::ptr::null(),
        library_member_type_cb: unsafe { core::mem::zeroed() },
        library_member_constant_cb: unsafe { core::mem::zeroed() },
        disabled_builtins: core::ptr::null(),
    };

    compile_or_throw_bytecode_builder_string_compile_options_parse_options(
        &mut bytecode_builder,
        &String::from(source),
        &options,
        &ParseOptions::default(),
    );

    let bytecode = bytecode_builder.get_bytecode();
    let bytecode_cstr = bytecode.as_str();

    let global_state = unsafe { lua_l_newstate() };
    let L: *mut lua_State = global_state;

    let result = unsafe {
        luau_load(
            L,
            b"source\0".as_ptr() as *const core::ffi::c_char,
            bytecode_cstr.as_ptr() as *const core::ffi::c_char,
            bytecode_cstr.len(),
            0,
        )
    };
    assert!(result == 0);

    let out = summarize_bytecode(L, -1, nesting_limit);

    unsafe {
        lua_close(L);
    }

    out
}
