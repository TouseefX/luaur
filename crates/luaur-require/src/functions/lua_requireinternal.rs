use crate::enums::status_require_impl::Status;
use crate::functions::lua_requirecont::{lua_requirecont, K_REQUIRE_STACK_VALUES};
use crate::functions::resolve_require::resolve_require;
use crate::records::luarequire_configuration::luarequire_Configuration;
use alloc::ffi::CString;
use core::ffi::{c_char, c_int};
use luaur_vm::enums::lua_status::lua_Status;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::functions::lua_error::lua_error;
use luaur_vm::functions::lua_getfield::lua_getfield;
use luaur_vm::functions::lua_gettop::lua_gettop;
use luaur_vm::functions::lua_l_findtable::luaL_findtable;
use luaur_vm::functions::lua_pushstring::lua_pushstring;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::functions::lua_settop::lua_settop;
use luaur_vm::functions::lua_tolightuserdata::lua_tolightuserdata;
use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::functions::lua_touserdata::lua_touserdata;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::functions::lua_yield::lua_yield;
use luaur_vm::macros::lua_l_checkstring::luaL_checkstring;
use luaur_vm::macros::lua_l_error::luaL_error;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::macros::lua_upvalueindex::lua_upvalueindex;
use luaur_vm::records::lua_state::lua_State;

const REGISTERED_CACHE_TABLE_KEY: *const c_char = c"_REGISTEREDMODULES".as_ptr();

pub fn lua_requireinternal(l: *mut lua_State, requirer_chunkname: *const c_char) -> i32 {
    unsafe {
        lua_settop(l, 1);

        let lrc = lua_touserdata(l, lua_upvalueindex(1)) as *mut luarequire_Configuration;
        if lrc.is_null() {
            luaL_error!(l, "unable to find require configuration");
            return 0;
        }

        let ctx = lua_tolightuserdata(l, lua_upvalueindex(2));
        let path = luaL_checkstring!(l, 1);

        luaL_findtable(l, LUA_REGISTRYINDEX, REGISTERED_CACHE_TABLE_KEY, 1);
        let path_bytes = core::ffi::CStr::from_ptr(path).to_bytes();
        let mut path_lower = path_bytes.to_vec();
        for byte in &mut path_lower {
            if *byte >= b'A' && *byte <= b'Z' {
                *byte += b'a' - b'A';
            }
        }
        let path_lower_c = CString::new(path_lower).unwrap();
        lua_getfield(l, -1, path_lower_c.as_ptr());
        if lua_type(l, -1) != lua_Type::LUA_TNIL as c_int {
            lua_remove(l, -2);
            return 1;
        }
        lua_pop(l, 2);

        let path_string = core::ffi::CStr::from_ptr(path)
            .to_string_lossy()
            .into_owned();
        let mut resolve_error = false;

        {
            let resolved_require = resolve_require(lrc, l, ctx, requirer_chunkname, path_string);

            if resolved_require.status == Status::Cached {
                return 1;
            }

            if resolved_require.status == Status::ErrorReported {
                let error = CString::new(resolved_require.error.replace('\0', "")).unwrap();
                lua_pushstring(l, error.as_ptr());
                resolve_error = true;
            } else {
                let cache_key = CString::new(resolved_require.cacheKey.replace('\0', "")).unwrap();
                let chunkname = CString::new(resolved_require.chunkname.replace('\0', "")).unwrap();
                let loadname = CString::new(resolved_require.loadname.replace('\0', "")).unwrap();
                lua_pushstring(l, cache_key.as_ptr());
                lua_pushstring(l, chunkname.as_ptr());
                lua_pushstring(l, loadname.as_ptr());
            }
        }

        if resolve_error {
            lua_error(l);
        }

        let stack_values = lua_gettop(l);
        luaur_common::LUAU_ASSERT!(stack_values == K_REQUIRE_STACK_VALUES);

        let chunkname = lua_tolstring(l, -2, core::ptr::null_mut());
        let loadname = lua_tolstring(l, -1, core::ptr::null_mut());

        let Some(load) = (*lrc).load else {
            luaL_error!(
                l,
                "require configuration is missing required function pointer: load"
            );
            return 0;
        };

        let num_results = load(l as *mut core::ffi::c_void, ctx, path, chunkname, loadname);
        if num_results == -1 {
            if lua_gettop(l) != stack_values {
                luaL_error!(l, "stack cannot be modified when require yields");
                return 0;
            }

            return lua_yield(l, 0);
        }

        lua_requirecont(l, lua_Status::LUA_OK as c_int)
    }
}
