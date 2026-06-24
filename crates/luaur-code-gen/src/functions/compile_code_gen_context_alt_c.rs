use crate::functions::compile_code_gen_context::compile_module_id_lua_state_i32_compilation_options_compilation_stats;
use crate::records::compilation_options::CompilationOptions;
use crate::records::compilation_result::CompilationResult;
use crate::records::compilation_stats::CompilationStats;
use crate::records::host_ir_hooks::HostIrHooks;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::module_id::ModuleId;
use core::ffi::c_int;

pub fn compile_lua_state_i32_i32_compilation_stats(
    l: *mut lua_State,
    idx: c_int,
    flags: u32,
    stats: *mut CompilationStats,
) -> CompilationResult {
    let options = CompilationOptions {
        flags,
        hooks: HostIrHooks::default(),
        userdata_types: core::ptr::null(),
        record_counters: false,
        nop_padding: false,
    };

    compile_module_id_lua_state_i32_compilation_options_compilation_stats(
        &ModuleId::default(),
        l,
        idx,
        &options,
        stats,
    )
}
