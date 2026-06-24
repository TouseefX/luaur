use alloc::string::String;
use core::ffi::c_char;

use luaur_code_gen::functions::get_assembly::get_assembly;
use luaur_code_gen::records::assembly_options::AssemblyOptions;
use luaur_code_gen::records::lowering_stats::LoweringStats;
use luaur_vm::functions::lua_close::lua_close;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::functions::luau_load::luau_load;
use luaur_vm::type_aliases::lua_state::lua_State;

struct LuaStateGuard(*mut lua_State);

impl Drop for LuaStateGuard {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe {
                lua_close(self.0);
            }
        }
    }
}

pub fn get_codegen_assembly(
    name: *const c_char,
    bytecode: &String,
    options: AssemblyOptions,
    stats: *mut LoweringStats,
) -> String {
    let global_state = LuaStateGuard(lua_l_newstate());
    let l = global_state.0;

    if unsafe {
        luau_load(
            l,
            name,
            bytecode.as_ptr() as *const c_char,
            bytecode.len(),
            0,
        )
    } == 0
    {
        unsafe { get_assembly(l, -1, options, stats) }
    } else {
        let name = unsafe { core::ffi::CStr::from_ptr(name).to_string_lossy() };
        eprintln!("Error loading bytecode {}", name);
        String::new()
    }
}
