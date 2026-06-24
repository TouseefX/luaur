use crate::functions::populate_rtti::populate_rtti;
use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::functions::freeze::freeze;
use luaur_analysis::functions::register_builtin_globals::register_builtin_globals;
use luaur_analysis::records::frontend::Frontend;
use luaur_analysis::records::frontend_options::FrontendOptions;
use luaur_analysis::records::null_file_resolver::NullFileResolver;
use luaur_analysis::records::null_module_resolver::NullModuleResolver;
use luaur_common::FFlag;
use luaur_vm::functions::lua_setfield::lua_setfield;
use luaur_vm::macros::lua_newtable::lua_newtable;
use luaur_vm::macros::lua_setglobal::lua_setglobal;
use luaur_vm::records::lua_state::lua_State;

pub unsafe extern "C" fn conformance_types_setup(l: *mut lua_State) {
    let _module_resolver = NullModuleResolver::new();
    let mut file_resolver = NullFileResolver::new();
    let mode = if FFlag::DebugLuauForceOldSolver.get() {
        SolverMode::Old
    } else {
        SolverMode::New
    };

    let mut frontend =
        Frontend::frontend_solver_mode_file_resolver_config_resolver_frontend_options(
            mode,
            &mut file_resolver.base,
            core::ptr::null_mut(),
            FrontendOptions::default(),
        );
    frontend.wire_self_pointers();

    let frontend_ptr = &mut frontend as *mut Frontend;
    register_builtin_globals(&mut *frontend_ptr, &mut (*frontend_ptr).globals, false);
    freeze((*frontend_ptr).globals.global_types_mut());

    lua_newtable(l);

    let global_scope = (*frontend_ptr).globals.global_scope();
    for (name, binding) in &global_scope.bindings {
        populate_rtti(l, binding.type_id);
        lua_setfield(l, -2, name.c_str());
    }

    lua_setglobal(l, c"RTTI".as_ptr());
}
