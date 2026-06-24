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
            let nargs = args.len() as c_int;
            // Guard against pushing more values than the Lua stack can hold:
            // an unprotected overflow would abort the VM. We need room for the
            // function + all arguments (+1 slack for the call machinery).
            if lua_checkstack(state, nargs.saturating_add(2)) == 0 {
                return Err(crate::error::Error::RuntimeError(
                    "stack overflow: too many arguments to function call".to_string(),
                ));
            }
            // Push the function, then the arguments.
            self.reference.push();
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

    /// Return a new function that, when called, prepends `args` to its own
    /// arguments and forwards to `self`.
    ///
    /// Mirrors `mlua::Function::bind`. Implemented as a Rust closure that
    /// captures the bound prefix and the target function.
    pub fn bind(&self, args: impl IntoLuaMulti) -> Result<Function> {
        let lua = self.lua();
        let bound: MultiValue = args.into_lua_multi(&lua)?;
        let target = self.clone();
        let bound_vec: Vec<crate::value::Value> = bound.into_vec();
        lua.create_function(move |_, extra: MultiValue| {
            let mut all = MultiValue::with_capacity(bound_vec.len() + extra.len());
            for v in &bound_vec {
                all.push_back(v.clone());
            }
            for v in extra {
                all.push_back(v);
            }
            target.call::<MultiValue>(all)
        })
    }

    /// A raw pointer identifying this function (for identity comparison).
    /// Mirrors `mlua::Function::to_pointer`.
    pub fn to_pointer(&self) -> *const std::ffi::c_void {
        let state = self.reference.state();
        unsafe {
            self.reference.push();
            let p = lua_topointer(state, -1);
            lua_pop(state, 1);
            p
        }
    }

    /// The function's environment table (its globals), or `None` for a Rust
    /// (C) function. Mirrors `mlua::Function::environment`.
    pub fn environment(&self) -> Option<crate::table::Table> {
        let lua = self.lua();
        let state = lua.state();
        unsafe {
            self.reference.push();
            // `lua_getfenv` only applies to Lua closures; a C function has no
            // accessible environment.
            if !self.is_lua_closure() {
                lua_pop(state, 1);
                return None;
            }
            lua_getfenv(state, -1);
            // stack: [func, env]
            if lua_type(state, -1) != ttype::TABLE {
                lua_pop(state, 2);
                return None;
            }
            let env = crate::table::Table::from_ref(lua.pop_ref());
            lua_pop(state, 1); // pop func
            Some(env)
        }
    }

    /// Set the function's environment table. Returns `Ok(false)` for a Rust
    /// (C) function (which has no settable environment) and `Ok(true)` for a
    /// Lua closure. Mirrors `mlua::Function::set_environment`.
    pub fn set_environment(&self, env: crate::table::Table) -> Result<bool> {
        let lua = self.lua();
        let state = lua.state();
        unsafe {
            self.reference.push();
            if !self.is_lua_closure() {
                lua_pop(state, 1);
                return Ok(false);
            }
            // stack: [func]; push env, then lua_setfenv(func_index).
            env.push_to_stack();
            let ok = lua_setfenv(state, -2);
            // lua_setfenv pops the env table; pop the function too.
            lua_pop(state, 1);
            Ok(ok != 0)
        }
    }

    /// Whether the value on top of the stack (this function, just pushed) is a
    /// Lua closure (vs a C function). Determined via the debug `what` field.
    unsafe fn is_lua_closure(&self) -> bool {
        let state = self.reference.state();
        unsafe {
            // The function is on top of the stack (index -1). Ask lua_getinfo
            // about it via the ">" level convention: push the function and use
            // option ">" so it pops the function and reads its info.
            lua_pushvalue(state, -1);
            let mut ar: LuaDebug = core::mem::zeroed();
            let opt = c">s";
            let ok = lua_getinfo(state, -1, opt.as_ptr() as *const c_char, &mut ar);
            if ok == 0 {
                return false;
            }
            if ar.what.is_null() {
                return false;
            }
            let what = std::ffi::CStr::from_ptr(ar.what).to_bytes();
            // "Lua" and "main" are Lua closures; "C" is a native function.
            what == b"Lua" || what == b"main"
        }
    }

    /// Debug information about this function. Mirrors `mlua::Function::info`.
    pub fn info(&self) -> FunctionInfo {
        let lua = self.lua();
        let state = lua.state();
        unsafe {
            self.reference.push();
            let mut ar: LuaDebug = core::mem::zeroed();
            // Options: n=name, s=source/what/linedefined, a=params/vararg,
            // u=upvalues. The ">" prefix pops the function from the stack and
            // reads info about it.
            let opt = c">nsau";
            let ok = lua_getinfo(state, -1, opt.as_ptr() as *const c_char, &mut ar);
            if ok == 0 {
                return FunctionInfo::default();
            }
            let cstr = |p: *const c_char| -> Option<String> {
                if p.is_null() {
                    None
                } else {
                    Some(
                        std::ffi::CStr::from_ptr(p)
                            .to_string_lossy()
                            .into_owned(),
                    )
                }
            };
            let what = cstr(ar.what).unwrap_or_default();
            let line_defined = if ar.linedefined > 0 {
                Some(ar.linedefined as i64)
            } else {
                None
            };
            // Lua chunks are loaded with a `=<name>` chunkname marker; mlua
            // reports the bare name in `source`, so strip a single leading
            // `=`/`@` for Lua/main functions. C functions keep their VM-reported
            // source verbatim (e.g. `=[C]`), matching mlua.
            let source = cstr(ar.source).map(|s| {
                if (what == "Lua" || what == "main")
                    && (s.starts_with('=') || s.starts_with('@'))
                {
                    s[1..].to_string()
                } else {
                    s
                }
            });
            FunctionInfo {
                name: cstr(ar.name),
                source,
                short_src: cstr(ar.short_src),
                line_defined,
                last_line_defined: None, // Luau does not report it.
                what,
                num_upvalues: ar.nupvals,
                num_params: ar.nparams,
                is_vararg: ar.isvararg != 0,
            }
        }
    }
}

/// Debug information about a [`Function`]. Mirrors `mlua::debug::FunctionInfo`
/// (the subset Luau reports).
#[derive(Clone, Debug, Default)]
pub struct FunctionInfo {
    /// The function's name, if known (Luau records the call-site name).
    pub name: Option<String>,
    /// The chunk source name (e.g. `"=[C]"` for native functions).
    pub source: Option<String>,
    /// A short, human-readable source description.
    pub short_src: Option<String>,
    /// The line where the function was defined, if it is a Lua function.
    pub line_defined: Option<i64>,
    /// The last line of the function's definition. Always `None` in Luau.
    pub last_line_defined: Option<i64>,
    /// `"Lua"`, `"C"`, or `"main"`.
    pub what: String,
    /// The number of upvalues.
    pub num_upvalues: u8,
    /// The number of fixed parameters.
    pub num_params: u8,
    /// Whether the function is variadic.
    pub is_vararg: bool,
}

impl std::fmt::Debug for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function")
    }
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        // Pointer identity (matches mlua): same underlying function object.
        self.to_pointer() == other.to_pointer()
    }
}
