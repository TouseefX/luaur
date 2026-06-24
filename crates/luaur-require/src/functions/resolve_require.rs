use crate::enums::status_require_impl::Status as RequireStatus;
use crate::enums::status_require_navigator::Status as NavigatorStatus;
use crate::records::luarequire_configuration::luarequire_Configuration;
use crate::records::navigator::Navigator;
use crate::records::resolved_require::ResolvedRequire;
use crate::records::runtime_error_handler::RuntimeErrorHandler;
use crate::records::runtime_navigation_context::RuntimeNavigationContext;
use alloc::ffi::CString;
use alloc::string::String;
use core::ffi::{c_char, c_int, c_void};
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::functions::lua_getfield::lua_getfield;
use luaur_vm::functions::lua_l_findtable::luaL_findtable;
use luaur_vm::functions::lua_remove::lua_remove;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::macros::lua_pop::lua_pop;
use luaur_vm::macros::lua_registryindex::LUA_REGISTRYINDEX;
use luaur_vm::records::lua_state::lua_State;

const REQUIRED_CACHE_TABLE_KEY: *const c_char = c"_MODULES".as_ptr();

pub fn resolve_require(
    lrc: *mut luarequire_Configuration,
    l: *mut lua_State,
    ctx: *mut c_void,
    requirer_chunkname: *const c_char,
    path: String,
) -> ResolvedRequire {
    unsafe {
        let Some(is_require_allowed) = lrc.as_ref().and_then(|config| config.is_require_allowed)
        else {
            return ResolvedRequire::from_error_message("require is not supported in this context");
        };

        if !is_require_allowed(l as *mut c_void, ctx, requirer_chunkname) {
            return ResolvedRequire::from_error_message("require is not supported in this context");
        }
    }

    let requirer_chunkname = unsafe {
        if requirer_chunkname.is_null() {
            String::new()
        } else {
            core::ffi::CStr::from_ptr(requirer_chunkname)
                .to_string_lossy()
                .into_owned()
        }
    };

    let mut navigation_context =
        RuntimeNavigationContext::new(lrc, l as *mut c_void, ctx, requirer_chunkname);
    let mut error_handler = RuntimeErrorHandler::new(path.clone());
    let mut navigator = Navigator::new(&mut navigation_context, &mut error_handler);

    if navigator.navigate(path) == NavigatorStatus::ErrorReported {
        return ResolvedRequire::resolved_require_from_error_handler(&error_handler);
    }

    if !navigation_context.is_module_present() {
        return ResolvedRequire::from_error_message("no module present at resolved path");
    }

    let Some(cache_key) = navigation_context.get_cache_key() else {
        return ResolvedRequire::from_error_message("could not get cache key for module");
    };

    unsafe {
        luaL_findtable(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY, 1);
        let cache_key_c = CString::new(cache_key.as_str()).unwrap();
        lua_getfield(l, -1, cache_key_c.as_ptr());
        let cached = lua_type(l, -1) != lua_Type::LUA_TNIL as c_int;
        lua_pop(l, 2);

        if cached {
            lua_getfield(l, LUA_REGISTRYINDEX, REQUIRED_CACHE_TABLE_KEY);
            lua_getfield(l, -1, cache_key_c.as_ptr());
            lua_remove(l, -2);

            return ResolvedRequire {
                status: RequireStatus::Cached,
                chunkname: String::new(),
                loadname: String::new(),
                cacheKey: String::new(),
                error: String::new(),
            };
        }
    }

    let Some(chunkname) = navigation_context.get_chunkname() else {
        return ResolvedRequire::from_error_message("could not get chunkname for module");
    };

    let Some(loadname) = navigation_context.get_loadname() else {
        return ResolvedRequire::from_error_message("could not get loadname for module");
    };

    ResolvedRequire {
        status: RequireStatus::ModuleRead,
        chunkname,
        loadname,
        cacheKey: cache_key,
        error: String::new(),
    }
}
