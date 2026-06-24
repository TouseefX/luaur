use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_getthreaddata::lua_getthreaddata;
use luaur_vm::functions::lua_mainthread::lua_mainthread;

#[allow(non_snake_case)]
pub unsafe fn get_type_function_runtime(L: *mut lua_State) -> *mut TypeFunctionRuntime {
    let main_thread = lua_mainthread(L as *mut luaur_vm::type_aliases::lua_state::lua_State);
    let data = lua_getthreaddata(main_thread);
    data as *mut TypeFunctionRuntime
}

// Pinned overload name advertised by the dependency cards.
#[allow(unused_imports, non_snake_case)]
pub use get_type_function_runtime as get_type_function_singleton_type;
