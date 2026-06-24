//! [`RegistryKey`] — long-term storage of a Lua value in the registry.
//!
//! Mirrors `mlua::RegistryKey`. A registry key holds a value reachable by the
//! GC for as long as the key (or a clone of it) is alive. It is created with
//! [`Lua::create_registry_value`] and read back with [`Lua::registry_value`].
//! Dropping the key (or calling [`Lua::remove_registry_value`]) releases the
//! registry slot.
//!
//! Under the hood a `RegistryKey` is just a public wrapper around the same
//! `lua_ref`/`lua_unref` machinery the internal handles already use
//! ([`crate::state::LuaRef`]): `create_registry_value` pushes the value and
//! takes a registry ref; `registry_value` re-pushes it. Each key remembers
//! which [`Lua`] minted it so a key used with the wrong instance is rejected
//! with [`Error::MismatchedRegistryKey`].

use crate::error::{Error, Result};
use crate::state::{Lua, LuaRef};
use crate::sync::{NotSync, XRc, NOT_SYNC};
use crate::traits::{FromLua, IntoLua};
use crate::value::Value;

/// An owned reference to a value stored in the Lua registry.
///
/// Mirrors `mlua::RegistryKey`. Cloning produces another handle to the **same**
/// stored value (the slot is shared via `Rc`). The value stays alive until the
/// last clone is dropped or it is explicitly removed.
///
/// Under the `send` feature it is `Send` but never `Sync` — see
/// [`crate::sync::NotSync`].
#[derive(Clone)]
pub struct RegistryKey {
    pub(crate) reference: XRc<LuaRef>,
    pub(crate) _not_sync: NotSync,
}

impl RegistryKey {
    pub(crate) fn from_ref(reference: LuaRef) -> RegistryKey {
        RegistryKey {
            reference: XRc::new(reference),
            _not_sync: NOT_SYNC,
        }
    }

    /// Push the stored value onto the owning state's stack.
    pub(crate) fn push(&self) {
        self.reference.push();
    }
}

impl std::fmt::Debug for RegistryKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RegistryKey")
    }
}

impl Lua {
    /// Store a value in the registry and return a [`RegistryKey`] that keeps it
    /// alive. Mirrors `mlua::Lua::create_registry_value`.
    pub fn create_registry_value(&self, value: impl IntoLua) -> Result<RegistryKey> {
        let v = value.into_lua(self)?;
        self.push_value(&v)?;
        Ok(RegistryKey::from_ref(self.pop_ref()))
    }

    /// Read back a value previously stored with [`Lua::create_registry_value`],
    /// converting it to `T`. Mirrors `mlua::Lua::registry_value`.
    pub fn registry_value<T: FromLua>(&self, key: &RegistryKey) -> Result<T> {
        if !self.owns_registry_value(key) {
            return Err(Error::MismatchedRegistryKey);
        }
        let state = self.state();
        let value = unsafe {
            key.push();
            let v = self.value_from_stack(-1)?;
            crate::ffi::lua_pop(state, 1);
            v
        };
        T::from_lua(value, self)
    }

    /// Remove a value from the registry, releasing its slot. Mirrors
    /// `mlua::Lua::remove_registry_value`.
    pub fn remove_registry_value(&self, key: RegistryKey) -> Result<()> {
        if !self.owns_registry_value(&key) {
            return Err(Error::MismatchedRegistryKey);
        }
        // Dropping the key releases the underlying `lua_ref` slot.
        drop(key);
        Ok(())
    }

    /// Replace the value stored under an existing key. Mirrors
    /// `mlua::Lua::replace_registry_value`.
    pub fn replace_registry_value(
        &self,
        key: &mut RegistryKey,
        value: impl IntoLua,
    ) -> Result<()> {
        if !self.owns_registry_value(key) {
            return Err(Error::MismatchedRegistryKey);
        }
        *key = self.create_registry_value(value)?;
        Ok(())
    }

    /// Whether this `Lua` instance owns `key` (i.e. `key` was minted by this VM,
    /// not a different one). Mirrors `mlua::Lua::owns_registry_value`.
    pub fn owns_registry_value(&self, key: &RegistryKey) -> bool {
        // Two `Lua` handles share the same VM iff their inner state pointers are
        // equal (cloning a `Lua` shares the `Rc<LuaInner>`; a separate
        // `Lua::new()` has a distinct state).
        key.reference.state() == self.state()
    }
}

// ---------------------------------------------------------------------------
// Conversions: a RegistryKey behaves like the value it stores when packed, and
// `Chunk::eval::<RegistryKey>()` stores the chunk's result.
// ---------------------------------------------------------------------------

impl IntoLua for RegistryKey {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        (&self).into_lua(lua)
    }
}

impl IntoLua for &RegistryKey {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        if self.reference.state() != lua.state() {
            return Err(Error::MismatchedRegistryKey);
        }
        let state = lua.state();
        unsafe {
            self.push();
            let v = lua.value_from_stack(-1)?;
            crate::ffi::lua_pop(state, 1);
            Ok(v)
        }
    }
}

impl FromLua for RegistryKey {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        lua.create_registry_value(value)
    }
}
