//! Faithful port of `void TypeFunctionRuntime::prepareState()`
//! (Analysis/src/TypeFunctionRuntime.cpp:230-248).
use crate::functions::register_type_user_data::register_type_user_data;
use crate::functions::register_types_library::register_types_library;
use crate::functions::set_type_function_environment::set_type_function_environment;
use crate::functions::type_function_alloc::type_function_alloc;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::lua_state::lua_State;
use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
use luaur_vm::functions::lua_l_sandboxthread::lua_l_sandboxthread;
use luaur_vm::functions::lua_newstate::lua_newstate;
use luaur_vm::functions::lua_setthreaddata::lua_setthreaddata;

/// `extern "C"` closer thunk for the `StateRef` deleter slot. C++ stores
/// `lua_close` as the `unique_ptr` deleter; the Rust `StateRef` deleter is
/// `extern "C" fn(*mut analysis::lua_State)`, so bridge to the VM's `lua_close`.
unsafe extern "C" fn lua_close_thunk(l: *mut lua_State) {
    luaur_vm::functions::lua_close::lua_close(l as *mut luaur_vm::records::lua_state::lua_State);
}

impl TypeFunctionRuntime {
    pub unsafe fn prepare_state(&mut self) {
        // if (state) return;
        if !self.state.0.is_null() {
            return;
        }

        // state = StateRef(lua_newstate(typeFunctionAlloc, nullptr), lua_close);
        let new_state =
            lua_newstate(Some(type_function_alloc), core::ptr::null_mut()) as *mut lua_State;
        self.state = (new_state, Some(lua_close_thunk));

        // lua_State* L = state.get();
        let l = self.state.0;
        let vm_l = l as *mut luaur_vm::records::lua_state::lua_State;

        // lua_setthreaddata(L, this);
        lua_setthreaddata(
            vm_l,
            self as *mut TypeFunctionRuntime as *mut core::ffi::c_void,
        );

        // setTypeFunctionEnvironment(L);
        set_type_function_environment(l);

        // registerTypeUserData(L);
        register_type_user_data(l);

        // registerTypesLibrary(L);
        register_types_library(l);

        // luaL_sandbox(L);
        lua_l_sandbox(vm_l);
        // luaL_sandboxthread(L);
        lua_l_sandboxthread(vm_l);
    }
}
