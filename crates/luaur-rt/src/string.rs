//! The [`LuaString`] handle. Mirrors `mlua::String`.

use std::rc::Rc;

use crate::error::{Error, Result};
use crate::ffi::*;
use crate::state::{Lua, LuaRef};

/// A garbage-collected Lua string.
///
/// Mirrors `mlua::String`. Holds a registry reference to the underlying Lua
/// string so the bytes stay alive for the handle's lifetime.
#[derive(Clone)]
pub struct LuaString {
    pub(crate) reference: Rc<LuaRef>,
}

impl LuaString {
    pub(crate) fn from_ref(reference: LuaRef) -> LuaString {
        LuaString {
            reference: Rc::new(reference),
        }
    }

    /// Push this string onto the owning state's stack.
    pub(crate) unsafe fn push_to_stack(&self) {
        self.reference.push();
    }

    /// Get the raw bytes of the string (a copy).
    ///
    /// Mirrors `mlua::String::as_bytes` (we return an owned `Vec<u8>` rather
    /// than a borrowed guard — a deliberate simplification).
    pub fn as_bytes(&self) -> Vec<u8> {
        let state = self.reference.state();
        unsafe {
            self.reference.push();
            let mut len = 0usize;
            let p = lua_tolstring(state, -1, &mut len);
            let bytes = if p.is_null() {
                Vec::new()
            } else {
                core::slice::from_raw_parts(p as *const u8, len).to_vec()
            };
            lua_pop(state, 1);
            bytes
        }
    }

    /// Get the string as a UTF-8 `&str`, erroring if it is not valid UTF-8.
    ///
    /// Mirrors `mlua::String::to_str` (returns an owned `String` here).
    pub fn to_str(&self) -> Result<String> {
        let bytes = self.as_bytes();
        String::from_utf8(bytes).map_err(|e| {
            Error::FromLuaConversionError {
                from: "string",
                to: "String".to_string(),
                message: Some(format!("invalid utf-8: {e}")),
            }
        })
    }

    /// Get the string lossily as a Rust `String` (invalid UTF-8 replaced).
    ///
    /// Mirrors `mlua::String::to_string_lossy`.
    pub fn to_string_lossy(&self) -> String {
        String::from_utf8_lossy(&self.as_bytes()).into_owned()
    }
}

impl std::fmt::Debug for LuaString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "String({:?})", self.to_string_lossy())
    }
}

impl PartialEq for LuaString {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

/// Helper to create a fresh Lua string from bytes on a given state, returning a
/// handle. Used by [`Lua::create_string`].
pub(crate) fn create_string(lua: &Lua, bytes: &[u8]) -> LuaString {
    let state = lua.state();
    unsafe {
        lua_pushlstring(state, bytes.as_ptr() as *const c_char, bytes.len());
        LuaString::from_ref(lua.pop_ref())
    }
}
