use alloc::string::String;
use core::ffi::CStr;

use crate::functions::load::load;
use crate::functions::serialize_table::serialize_table;
use crate::records::config_table::ConfigTable;
use crate::records::interrupt_callbacks::InterruptCallbacks;
use luaur_vm::enums::lua_status::lua_Status;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::functions::lua_callbacks::lua_callbacks;
use luaur_vm::functions::lua_close::lua_close;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_newstate::lua_l_newstate;
use luaur_vm::functions::lua_l_openlibs::lua_l_openlibs;
use luaur_vm::functions::lua_l_sandbox::lua_l_sandbox;
use luaur_vm::functions::lua_resume::lua_resume;
use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::type_aliases::lua_state::lua_State;

pub fn extract_config(
    source: &String,
    callbacks: &InterruptCallbacks,
    error: &mut String,
) -> Option<ConfigTable> {
    struct StateGuard(*mut lua_State);

    impl Drop for StateGuard {
        fn drop(&mut self) {
            unsafe {
                if !self.0.is_null() {
                    lua_close(self.0);
                }
            }
        }
    }

    let state = StateGuard(lua_l_newstate());
    let l = state.0;

    unsafe {
        lua_l_openlibs(l);
        lua_l_sandbox(l);
    }

    if let Some(load_error) = load(l, source) {
        *error = load_error;
        return None;
    }

    if let Some(init_callback) = &callbacks.init_callback {
        init_callback(l);
    }

    unsafe {
        (*lua_callbacks(l)).interrupt = callbacks.interrupt_callback;

        match lua_resume(l, core::ptr::null_mut(), 0) {
            status if status == lua_Status::LUA_OK as i32 => {}
            status
                if status == lua_Status::LUA_BREAK as i32
                    || status == lua_Status::LUA_YIELD as i32 =>
            {
                *error = String::from("configuration execution cannot yield");
                return None;
            }
            _ => {
                *error = lua_string(l, -1);
                return None;
            }
        }

        if lua_gettop(l) != 1 {
            *error = String::from("configuration must return exactly one value");
            return None;
        }

        if lua_type(l, -1) != lua_Type::LUA_TTABLE as i32 {
            *error = String::from("configuration did not return a table");
            return None;
        }
    }

    serialize_table(l, error)
}

unsafe fn lua_string(l: *mut lua_State, index: i32) -> String {
    let ptr = lua_tolstring(l, index, core::ptr::null_mut());
    if ptr.is_null() {
        String::new()
    } else {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}
