//! The [`Function`] handle. Mirrors `mlua::Function`.

use std::rc::Rc;

use crate::error::Result;
use crate::ffi::*;
use crate::multi::MultiValue;
use crate::state::{Lua, LuaRef};
use crate::traits::{FromLuaMulti, IntoLuaMulti};

/// A handle to a callable Lua value (a Lua closure or a Rust function).
///
/// Mirrors `mlua::Function`.
#[derive(Clone)]
pub struct Function {
    pub(crate) reference: Rc<LuaRef>,
}

impl Function {
    pub(crate) fn from_ref(reference: LuaRef) -> Function {
        Function {
            reference: Rc::new(reference),
        }
    }

    pub(crate) unsafe fn push_to_stack(&self) {
        self.reference.push();
    }

    /// The owning [`Lua`].
    pub fn lua(&self) -> Lua {
        self.reference.lua()
    }

    /// Call the function with `args`, converting the results to `R`.
    ///
    /// Mirrors `mlua::Function::call`. Runs under `lua_pcall`, so a Lua runtime
    /// error (or a Rust callback returning `Err`) becomes `Err(Error)` rather
    /// than unwinding.
    pub fn call<R: FromLuaMulti>(&self, args: impl IntoLuaMulti) -> Result<R> {
        let lua = self.lua();
        let state = lua.state();
        let args: MultiValue = args.into_lua_multi(&lua)?;

        unsafe {
            let base = lua_gettop(state);
            // Push the function, then the arguments.
            self.reference.push();
            let nargs = args.len() as c_int;
            for v in args.iter() {
                lua.push_value(v)?;
            }
            // LUA_MULTRET == -1: keep every result.
            let status = lua_pcall(state, nargs, -1, 0);
            if status != 0 {
                return Err(lua.pop_error(status));
            }
            // Collect every value pushed above `base` as the results.
            let top = lua_gettop(state);
            let nresults = top - base;
            let mut results = MultiValue::with_capacity(nresults.max(0) as usize);
            for i in 0..nresults {
                let idx = base + 1 + i;
                results.push_back(lua.value_from_stack(idx)?);
            }
            lua_settop(state, base);
            R::from_lua_multi(results, &lua)
        }
    }
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function")
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.reference.state() == other.reference.state()
            && self.reference.id() == other.reference.id()
    }
}
