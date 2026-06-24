use crate::records::compilation_options::CompilationOptions;
use crate::records::compilation_result::CompilationResult;
use crate::records::compilation_stats::CompilationStats;
use crate::type_aliases::lua_state::lua_State;
use crate::type_aliases::module_id::ModuleId;

extern "C" {
    #[link_name = "_ZN4Luau7CodeGen15compileInternalERKSt5arrayIhLm16EEP9lua_StateiRKNS0_18CompilationOptionsEPNS0_16CompilationStatsE"]
    fn compile_internal_impl(
        module_id: *const ModuleId,
        l: *mut lua_State,
        idx: i32,
        options: *const CompilationOptions,
        stats: *mut CompilationStats,
    ) -> CompilationResult;
}

pub fn compile_module_id_lua_state_i32_compilation_options_compilation_stats(
    module_id: &ModuleId,
    l: *mut lua_State,
    idx: i32,
    options: &CompilationOptions,
    stats: *mut CompilationStats,
) -> CompilationResult {
    unsafe { compile_internal_impl(module_id, l, idx, options, stats) }
}
