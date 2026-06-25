//! (De)serialization support using [`serde`].
//!
//! This module mirrors `mlua`'s `serde` feature: it adds the [`LuaSerdeExt`]
//! trait to [`Lua`], a serde [`Serializer`] that builds Lua [`Value`]s, and a
//! [`Deserializer`] that reads them. It is gated behind the `serde` cargo
//! feature; with the feature off the rest of the crate is byte-for-byte
//! unchanged.
//!
//! ## Luau deviations from mlua's Lua-version-specific behavior
//!
//! - **Numbers are `f64`.** Luau has no distinct integer subtype at the VM
//!   level; luaur-rt reconstructs [`Value::Integer`] vs [`Value::Number`] from
//!   whether an `f64` is an exact, in-range whole number (see
//!   [`crate::Value`]). Serialization therefore treats an integral `f64` the
//!   same as an integer — matching mlua's *observable* JSON output for the
//!   values under test.
//! - **`null` sentinel.** mlua encodes a serde "none"/JSON `null` using a
//!   `LightUserData(NULL)` value. luaur-rt's [`Value`] has no `LightUserData`
//!   variant, so [`LuaSerdeExt::null`] instead returns a dedicated, per-`Lua`
//!   **sentinel table** (a unique empty table cached in the state). Serde
//!   recognizes this exact table by pointer identity and treats it as
//!   `null`/`None`, reproducing mlua's behavior at the API level.

use std::cell::RefCell;
use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

use crate::error::Result;
use crate::state::Lua;
use crate::sys::lua_State;
use crate::table::Table;
use crate::value::Value;

pub mod de;
pub mod ser;
mod value_serialize;

pub use de::{Deserializer, Options as DeserializeOptions};
pub use ser::{Options as SerializeOptions, Serializer};
pub use value_serialize::{SerializableTable, SerializableValue};

// ---------------------------------------------------------------------------
// Per-state sentinels (the `null` table and the array metatable).
//
// mlua keeps these in the Lua registry. luaur-rt's `Value` has no
// `LightUserData`, so the `null` value is modelled as a dedicated, unique empty
// table per `Lua`; the array metatable is likewise a unique table per `Lua`.
//
// Both are cached in a thread-local keyed by the raw state pointer. `Lua` is
// `!Send`/`!Sync` (single-threaded), so a thread-local is sound: every handle
// to a given VM lives on the same thread. Each cached `Table` holds its own
// registry reference, keeping the underlying Lua object alive for the lifetime
// of the cache entry. This whole mechanism is compiled only under the `serde`
// feature.
// ---------------------------------------------------------------------------

struct StateSentinels {
    null: Table,
    array_metatable: Table,
}

thread_local! {
    static SENTINELS: RefCell<HashMap<*mut lua_State, StateSentinels>> =
        RefCell::new(HashMap::new());
}

/// Returns the per-`Lua` `null` sentinel table, creating it on first use.
pub(crate) fn null_table(lua: &Lua) -> Table {
    let key = lua.state();
    SENTINELS.with(|cell| {
        cell.borrow_mut()
            .entry(key)
            .or_insert_with(|| StateSentinels {
                null: lua.create_table(),
                array_metatable: build_array_metatable(lua),
            })
            .null
            .clone()
    })
}

/// Returns the per-`Lua` array metatable, creating it on first use.
pub(crate) fn array_metatable_table(lua: &Lua) -> Table {
    let key = lua.state();
    SENTINELS.with(|cell| {
        cell.borrow_mut()
            .entry(key)
            .or_insert_with(|| StateSentinels {
                null: lua.create_table(),
                array_metatable: build_array_metatable(lua),
            })
            .array_metatable
            .clone()
    })
}

/// Build the array metatable: a fresh table with `__metatable = false` so Lua
/// code cannot read or replace it (mirrors mlua, which protects the metatable
/// the same way).
fn build_array_metatable(lua: &Lua) -> Table {
    let mt = lua.create_table();
    // Best-effort: if this ever failed we'd still have a usable (if
    // unprotected) metatable, so ignore the (impossible here) error.
    let _ = mt.raw_set("__metatable", false);
    mt
}

/// Whether `value` is the `null` sentinel for its owning `Lua` (compared by
/// table pointer identity).
pub(crate) fn is_null(value: &Value) -> bool {
    if let Value::Table(t) = value {
        let key = t.lua().state();
        return SENTINELS.with(|cell| {
            cell.borrow()
                .get(&key)
                .map(|s| s.null.to_pointer() == t.to_pointer())
                .unwrap_or(false)
        });
    }
    false
}

/// Whether `table` carries the array metatable (compared by pointer identity).
pub(crate) fn has_array_metatable(table: &Table) -> bool {
    let key = table.lua().state();
    let array_ptr = SENTINELS.with(|cell| {
        cell.borrow()
            .get(&key)
            .map(|s| s.array_metatable.to_pointer())
    });
    match array_ptr {
        Some(ptr) => table
            .metatable()
            .map(|mt| mt.to_pointer() == ptr)
            .unwrap_or(false),
        None => false,
    }
}

/// Trait for serializing/deserializing Lua values using Serde. Mirrors
/// `mlua::LuaSerdeExt`.
pub trait LuaSerdeExt {
    /// A special value used to encode/decode optional (none) values.
    ///
    /// In luaur-rt this is a dedicated, per-`Lua` sentinel [`Table`] (see the
    /// module docs); mlua uses a `LightUserData(NULL)`. The observable behavior
    /// — `null` round-trips to/from serde `None`/JSON `null` — is the same.
    fn null(&self) -> Value;

    /// A metatable attachable to a Lua table to systematically encode it as an
    /// array (instead of a map). The encoded array contains only the sequence
    /// part of the table, with the same length as the `#` operator.
    fn array_metatable(&self) -> Table;

    /// Converts `T` into a [`Value`] instance.
    fn to_value<T: Serialize + ?Sized>(&self, t: &T) -> Result<Value>;

    /// Converts `T` into a [`Value`] instance with options.
    fn to_value_with<T>(&self, t: &T, options: ser::Options) -> Result<Value>
    where
        T: Serialize + ?Sized;

    /// Deserializes a [`Value`] into any serde-deserializable object.
    #[allow(clippy::wrong_self_convention)]
    fn from_value<T: DeserializeOwned>(&self, value: Value) -> Result<T>;

    /// Deserializes a [`Value`] into any serde-deserializable object with
    /// options.
    #[allow(clippy::wrong_self_convention)]
    fn from_value_with<T: DeserializeOwned>(&self, value: Value, options: de::Options)
        -> Result<T>;
}

impl LuaSerdeExt for Lua {
    fn null(&self) -> Value {
        Value::Table(null_table(self))
    }

    fn array_metatable(&self) -> Table {
        array_metatable_table(self)
    }

    fn to_value<T>(&self, t: &T) -> Result<Value>
    where
        T: Serialize + ?Sized,
    {
        t.serialize(ser::Serializer::new(self))
    }

    fn to_value_with<T>(&self, t: &T, options: ser::Options) -> Result<Value>
    where
        T: Serialize + ?Sized,
    {
        t.serialize(ser::Serializer::new_with_options(self, options))
    }

    fn from_value<T>(&self, value: Value) -> Result<T>
    where
        T: DeserializeOwned,
    {
        T::deserialize(de::Deserializer::new(value))
    }

    fn from_value_with<T>(&self, value: Value, options: de::Options) -> Result<T>
    where
        T: DeserializeOwned,
    {
        T::deserialize(de::Deserializer::new_with_options(value, options))
    }
}
