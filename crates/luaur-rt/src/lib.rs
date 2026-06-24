//! # luaur-rt
//!
//! A safe, ergonomic, **mlua-style** high-level API for
//! [`luaur`](https://github.com/pjankiewicz/luaur) — a pure-Rust translation of
//! Roblox's [Luau](https://github.com/luau-lang/luau).
//!
//! The public surface deliberately mirrors [`mlua`](https://docs.rs/mlua)'s
//! interface — the same type names ([`Lua`], [`Value`], [`Table`],
//! [`Function`], [`LuaString`], [`MultiValue`], [`Variadic`], [`Error`]), the
//! same method names and call shapes (`Lua::new`, `lua.globals()`,
//! `lua.create_function`, `lua.load(src).eval::<T>()`, `table.set/get`,
//! `function.call::<R>(args)`), and the same conversion traits ([`FromLua`],
//! [`IntoLua`], [`FromLuaMulti`], [`IntoLuaMulti`]) and userdata traits
//! ([`UserData`], [`UserDataMethods`]). The *implementation*, however, is
//! entirely original: it is written directly over luaur's pure-Rust C API
//! (`lua_*`), not over a C FFI.
//!
//! ```
//! use luaur_rt::prelude::*;
//!
//! let lua = Lua::new();
//! let add = lua
//!     .create_function(|_, (a, b): (i64, i64)| Ok(a + b))
//!     .unwrap();
//! lua.globals().set("add", add).unwrap();
//! let sum: i64 = lua.load("return add(2, 3)").eval().unwrap();
//! assert_eq!(sum, 5);
//! ```
//!
//! ## Single-threaded
//!
//! Like mlua's default, [`Lua`] is single-threaded: it is built on `Rc`, so it
//! is neither `Send` nor `Sync`. Clone a [`Lua`] to get another handle to the
//! same VM.
//!
//! ## Deferred (not yet implemented)
//!
//! The following parts of mlua's surface are intentionally **out of scope** and
//! are noted here rather than implemented:
//!
//! - `async` support (P4).
//! - `serde` integration (P4).
//! - Multi-VM `Send`/`Sync` (`WeakLua`, send-able handles) (P4).
//! - Thread event callbacks (`ThreadEvent`/`ThreadTriggers`/
//!   `set_thread_event_callback`) and per-thread hooks.
//! - The `chunk!` proc-macro and `#[derive(UserData)]` (P4).
//!
//! Implemented in Phase 1: `Thread`/coroutine wrappers, public `RegistryKey`
//! storage, `UserDataFields`, typed `AnyUserData::borrow`/`borrow_mut`/`take`/
//! `is`, the `MetaMethod` enum, and `Function::info`/`environment`.
//!
//! Implemented in Phase 2: the Luau-specific runtime types [`Buffer`] (the
//! `buffer` type) and [`Vector`] (the `vector` type), with their
//! [`Value::Buffer`]/[`Value::Vector`] variants and `FromLua`/`IntoLua` impls.

#![forbid(unsafe_op_in_unsafe_fn)]

mod buffer;
mod callback;
mod chunk;
mod conversion;
mod error;
mod ffi;
mod function;
mod metamethod;
mod multi;
mod registry;
mod scope;
mod state;
mod string;
mod table;
mod thread;
mod traits;
mod userdata;
mod value;
mod vector;

pub use buffer::Buffer;
pub use chunk::{Chunk, ChunkMode};
pub use error::{Error, ExternalError, ExternalResult, Result};
pub use function::{Function, FunctionInfo};
pub use metamethod::MetaMethod;
pub use multi::{MultiValue, Variadic};
pub use registry::RegistryKey;
pub use scope::Scope;
pub use state::Lua;
pub use string::LuaString;
pub use table::{Table, TablePairs, TableSequence};
pub use thread::{Thread, ThreadStatus};
pub use traits::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti};
pub use userdata::{
    AnyUserData, UserData, UserDataFields, UserDataMethods, UserDataRef, UserDataRefMut,
};
pub use value::{Integer, Number, Value};
pub use vector::Vector;

/// Idiomatic glob-import prelude. Mirrors `mlua::prelude`, additionally
/// re-exporting the short names so `use luaur_rt::prelude::*;` brings the whole
/// ergonomic surface into scope.
pub mod prelude {
    pub use crate::{
        AnyUserData, Buffer, Chunk, Error, ExternalError, ExternalResult, FromLua, FromLuaMulti,
        Function, IntoLua, IntoLuaMulti, Lua, LuaString, MetaMethod, MultiValue, RegistryKey,
        Result, Scope, Table, Thread, ThreadStatus, UserData, UserDataFields, UserDataMethods,
        Value, Variadic, Vector,
    };

    // mlua-style `Lua*`-prefixed aliases for users coming from mlua's prelude.
    pub use crate::AnyUserData as LuaAnyUserData;
    pub use crate::Error as LuaError;
    pub use crate::Function as LuaFunction;
    pub use crate::MetaMethod as LuaMetaMethod;
    pub use crate::MultiValue as LuaMultiValue;
    pub use crate::RegistryKey as LuaRegistryKey;
    pub use crate::Result as LuaResult;
    pub use crate::Table as LuaTable;
    pub use crate::Thread as LuaThread;
    pub use crate::ThreadStatus as LuaThreadStatus;
    pub use crate::UserData as LuaUserData;
    pub use crate::UserDataFields as LuaUserDataFields;
    pub use crate::UserDataMethods as LuaUserDataMethods;
    pub use crate::Value as LuaValue;
    pub use crate::Variadic as LuaVariadic;
    // `LuaString` already carries the `Lua` prefix.
}

#[cfg(test)]
mod tests;
