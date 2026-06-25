//! Luau-specific `Lua` extensions: sandboxing, safeenv, fflags, and the
//! per-VM compiler. Mirrors the Luau-only parts of mlua's `Lua` surface.

use std::cell::RefCell;
use std::collections::HashMap;

use crate::compiler::Compiler;
use crate::error::{Error, Result};
use crate::ffi::*;
use crate::state::Lua;
use crate::table::Table;
use crate::thread::Thread;

thread_local! {
    /// Per-VM saved *original* globals table (used to revert `sandbox(false)`),
    /// keyed by the global-state pointer.
    static SANDBOX_SAVED_GLOBALS: RefCell<HashMap<*mut core::ffi::c_void, Table>> =
        RefCell::new(HashMap::new());

    /// Per-VM compiler installed via `Lua::set_compiler`, keyed by global state.
    static VM_COMPILERS: RefCell<HashMap<*mut core::ffi::c_void, Compiler>> =
        RefCell::new(HashMap::new());
}

unsafe fn global_key(state: *mut lua_State) -> *mut core::ffi::c_void {
    unsafe { (*state).global as *mut core::ffi::c_void }
}

impl Lua {
    /// Enable or disable sandbox mode. Mirrors `mlua::Lua::sandbox`.
    ///
    /// Enabling sets every library table (and the globals table) read-only and
    /// activates `safeenv`, then installs a fresh proxy global table (via
    /// `luaL_sandboxthread`) so that script-level global writes go to a
    /// throwaway table whose `__index` is the original environment. Disabling
    /// restores the original globals table and clears the read-only/safeenv
    /// flags.
    ///
    /// **DEVIATION:** Luau's standard library (as bundled in luaur) does not
    /// register `collectgarbage`; mlua's sandbox test additionally checks that
    /// `collectgarbage` is restricted under the sandbox. That part is not
    /// exercisable here (see `tests/mlua_luau.rs`).
    pub fn sandbox(&self, enabled: bool) -> Result<()> {
        let state = self.state();
        let key = unsafe { global_key(state) };
        unsafe {
            if enabled {
                // Save the original globals table so we can restore it later.
                let original = self.globals();
                SANDBOX_SAVED_GLOBALS.with(|m| {
                    m.borrow_mut().entry(key).or_insert(original);
                });
                // Make libraries + base metatables read-only and set safeenv.
                lua_l_sandbox(state);
                // Install the proxy global table for script-level writes.
                lua_l_sandboxthread(state);
            } else {
                // Restore the original globals table (dropping the proxy and any
                // globals written into it).
                let saved = SANDBOX_SAVED_GLOBALS.with(|m| m.borrow_mut().remove(&key));
                if let Some(orig) = saved {
                    orig.push_to_stack();
                    lua_replace(state, LUA_GLOBALSINDEX);
                    // Clear read-only + safeenv on the restored globals so it is
                    // writable again.
                    lua_setreadonly(state, LUA_GLOBALSINDEX, 0);
                    lua_setsafeenv(state, LUA_GLOBALSINDEX, 0);
                    // Also clear read-only on the library tables.
                    self.clear_library_readonly();
                }
            }
        }
        Ok(())
    }

    /// Clear the read-only flag on every library table reachable from the
    /// (restored) globals. Used when leaving sandbox mode.
    fn clear_library_readonly(&self) {
        let globals = self.globals();
        if let Ok(pairs) = globals
            .pairs::<crate::value::Value, crate::value::Value>()
            .collect::<Result<Vec<_>>>()
        {
            for (_, v) in pairs {
                if let crate::value::Value::Table(t) = v {
                    t.set_readonly(false);
                }
            }
        }
    }

    /// Set or clear the `safeenv` flag on the globals table. Mirrors
    /// `mlua::Globals::set_safeenv` applied to the main globals.
    ///
    /// `safeenv` lets the VM fast-path global reads; clearing it forces the slow
    /// path (needed when globals/`__index` may change at runtime).
    pub fn set_safeenv(&self, enabled: bool) {
        let state = self.state();
        unsafe {
            lua_setsafeenv(state, LUA_GLOBALSINDEX, enabled as c_int);
        }
    }

    /// Install a default [`Compiler`] used to compile every chunk loaded by this
    /// VM (unless a chunk overrides it via
    /// [`Chunk::set_compiler`](crate::Chunk::set_compiler)). Mirrors
    /// `mlua::Lua::set_compiler`.
    pub fn set_compiler(&self, compiler: Compiler) {
        let state = self.state();
        let key = unsafe { global_key(state) };
        VM_COMPILERS.with(|m| {
            m.borrow_mut().insert(key, compiler);
        });
    }

    /// The VM-default compiler installed via [`Lua::set_compiler`], if any.
    pub(crate) fn vm_compiler(&self) -> Option<Compiler> {
        let state = self.state();
        let key = unsafe { global_key(state) };
        VM_COMPILERS.with(|m| m.borrow().get(&key).cloned())
    }

    /// Set (or clear) the metatable shared by all values of a Luau built-in
    /// type `T`. Mirrors `mlua::Lua::set_type_metatable`.
    ///
    /// Currently implemented for [`Vector`](crate::Vector) (the only built-in
    /// type whose shared metatable mlua's tests exercise). Setting it installs
    /// a metatable in the VM's per-type metatable slot, so e.g. `v.x`/`v:method`
    /// dispatch through it.
    pub fn set_type_metatable<T: TypeMetatable>(&self, metatable: Option<Table>) {
        T::set_type_metatable(self, metatable);
    }

    /// Set a Luau fast-flag (FFlag) by name. Mirrors `mlua::Lua::set_fflag`.
    ///
    /// **DEVIATION:** luaur's FastFlags are a fixed, compile-time `FFlag` enum
    /// rather than a string-keyed registry, so there is no way to look a flag up
    /// by an arbitrary name. This therefore always reports the name as unknown
    /// (`Err`) — which matches mlua's contract for an unrecognized flag (the
    /// only behavior its `test_fflags` asserts). Known flags are configured at
    /// VM-construction time via `luaur_common::set_all_flags`.
    pub fn set_fflag(name: &str, _enabled: bool) -> Result<()> {
        Err(Error::runtime(format!("fflag '{name}' is not supported")))
    }
}

impl Thread {
    /// Sandbox this coroutine: install a fresh proxy global table on its own
    /// state so global writes inside the coroutine stay local to it. Mirrors
    /// `mlua::Thread::sandbox`.
    pub fn sandbox(&self) -> Result<()> {
        let co = self.thread_state;
        unsafe {
            lua_l_sandboxthread(co);
        }
        Ok(())
    }
}

/// Luau built-in types that have a shared, per-type metatable settable via
/// [`Lua::set_type_metatable`]. Mirrors mlua's sealed `LuauType` trait.
pub trait TypeMetatable: private::Sealed {
    /// Install (or clear) the shared metatable for this type.
    fn set_type_metatable(lua: &Lua, metatable: Option<Table>);
}

mod private {
    pub trait Sealed {}
    impl Sealed for crate::vector::Vector {}
}

impl TypeMetatable for crate::vector::Vector {
    fn set_type_metatable(lua: &Lua, metatable: Option<Table>) {
        let state = lua.state();
        unsafe {
            // Push a vector value, then the metatable (or nil), and call
            // `lua_setmetatable`: for a non-table/non-userdata value it stores
            // the metatable in the VM's global per-type slot (`g->mt[VECTOR]`).
            crate::ffi::lua_pushvector_lua_state_f32_f32_f32_f32(state, 0.0, 0.0, 0.0, 0.0);
            match metatable {
                Some(mt) => mt.push_to_stack(),
                None => crate::ffi::lua_pushnil(state),
            }
            crate::ffi::lua_setmetatable(state, -2);
            // Pop the vector value left on the stack.
            crate::ffi::lua_pop(state, 1);
        }
    }
}
