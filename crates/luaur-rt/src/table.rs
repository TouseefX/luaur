//! The [`Table`] handle. Mirrors `mlua::Table`.

use std::rc::Rc;

use crate::error::Result;
use crate::ffi::*;
use crate::state::{Lua, LuaRef};
use crate::traits::{FromLua, IntoLua};
use crate::value::Value;

/// A handle to a Lua table.
///
/// Mirrors `mlua::Table`. Holds a registry reference keeping the table alive.
#[derive(Clone)]
pub struct Table {
    pub(crate) reference: Rc<LuaRef>,
}

impl Table {
    pub(crate) fn from_ref(reference: LuaRef) -> Table {
        Table {
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

    /// Set `table[key] = value`, honoring metamethods (`__newindex`).
    ///
    /// Mirrors `mlua::Table::set`.
    pub fn set<K: IntoLua, V: IntoLua>(&self, key: K, value: V) -> Result<()> {
        let lua = self.lua();
        let state = lua.state();
        let k = key.into_lua(&lua)?;
        let v = value.into_lua(&lua)?;
        unsafe {
            self.reference.push(); // table
            lua.push_value(&k)?; // key
            lua.push_value(&v)?; // value
            lua_settable(state, -3);
            lua_pop(state, 1); // pop table
        }
        Ok(())
    }

    /// Get `table[key]`, honoring metamethods (`__index`), converting the
    /// result to `V`.
    ///
    /// Mirrors `mlua::Table::get`.
    pub fn get<K: IntoLua, V: FromLua>(&self, key: K) -> Result<V> {
        let lua = self.lua();
        let state = lua.state();
        let k = key.into_lua(&lua)?;
        let value = unsafe {
            self.reference.push(); // table
            lua.push_value(&k)?; // key
            lua_gettable(state, -2); // replaces key with value
            let v = lua.value_from_stack(-1)?;
            lua_pop(state, 2); // pop value + table
            v
        };
        V::from_lua(value, &lua)
    }

    /// Whether `table[key]` is non-nil.
    ///
    /// Mirrors `mlua::Table::contains_key`.
    pub fn contains_key<K: IntoLua>(&self, key: K) -> Result<bool> {
        let v: Value = self.get(key)?;
        Ok(!v.is_nil())
    }

    /// The border length (`#table`).
    ///
    /// Mirrors `mlua::Table::raw_len` (returns `usize`). luaur's `lua_objlen`
    /// gives the same border-length semantics as `lua_rawlen`.
    pub fn raw_len(&self) -> usize {
        let state = self.reference.state();
        unsafe {
            self.reference.push();
            let n = lua_objlen(state, -1);
            lua_pop(state, 1);
            n.max(0) as usize
        }
    }

    /// The length (`#table`).
    ///
    /// Mirrors `mlua::Table::len` (returns `Result<usize>` — infallible here
    /// since we use the raw border length).
    pub fn len(&self) -> Result<usize> {
        Ok(self.raw_len())
    }

    /// Whether the table's sequence part is empty.
    pub fn is_empty(&self) -> bool {
        self.raw_len() == 0
    }

    /// Iterate over `(key, value)` pairs.
    ///
    /// Mirrors `mlua::Table::pairs`. Returns an iterator yielding `Result<(K,
    /// V)>` items. Uses `lua_next` under the hood.
    pub fn pairs<K: FromLua, V: FromLua>(&self) -> TablePairs<K, V> {
        TablePairs {
            table: self.clone(),
            next_key: Some(Value::Nil),
            _phantom: std::marker::PhantomData,
        }
    }

    /// Collect all `(key, value)` pairs into a `Vec`. Convenience over
    /// [`Table::pairs`].
    pub fn pairs_vec<K: FromLua, V: FromLua>(&self) -> Result<Vec<(K, V)>> {
        self.pairs().collect()
    }
}

impl std::fmt::Debug for Table {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Table(len={})", self.raw_len())
    }
}

impl PartialEq for Table {
    fn eq(&self, other: &Self) -> bool {
        // Pointer identity: same registry slot and same state.
        self.reference.state() == other.reference.state()
            && self.reference.id() == other.reference.id()
    }
}

/// Iterator over a table's key/value pairs (see [`Table::pairs`]).
pub struct TablePairs<K, V> {
    table: Table,
    next_key: Option<Value>,
    _phantom: std::marker::PhantomData<(K, V)>,
}

impl<K: FromLua, V: FromLua> Iterator for TablePairs<K, V> {
    type Item = Result<(K, V)>;

    fn next(&mut self) -> Option<Self::Item> {
        let key = self.next_key.take()?;
        let lua = self.table.lua();
        let state = lua.state();
        unsafe {
            self.table.reference.push(); // [.. table]
            if lua.push_value(&key).is_err() {
                lua_pop(state, 1);
                return None;
            }
            // stack: [table, key]
            let has = lua_next(state, -2);
            if has == 0 {
                // lua_next popped the key; pop the table.
                lua_pop(state, 1);
                self.next_key = None;
                return None;
            }
            // stack: [table, next_key, value]
            let k_val = match lua.value_from_stack(-2) {
                Ok(v) => v,
                Err(e) => {
                    lua_pop(state, 3);
                    return Some(Err(e));
                }
            };
            let v_val = match lua.value_from_stack(-1) {
                Ok(v) => v,
                Err(e) => {
                    lua_pop(state, 3);
                    return Some(Err(e));
                }
            };
            // Remember the key for the next iteration, then clean the stack.
            self.next_key = Some(k_val.clone());
            lua_pop(state, 3); // value, next_key, table

            let k = match K::from_lua(k_val, &lua) {
                Ok(k) => k,
                Err(e) => return Some(Err(e)),
            };
            let v = match V::from_lua(v_val, &lua) {
                Ok(v) => v,
                Err(e) => return Some(Err(e)),
            };
            Some(Ok((k, v)))
        }
    }
}

/// Create a fresh empty table on `lua` and return a handle.
pub(crate) fn create_table(lua: &Lua) -> Table {
    let state = lua.state();
    unsafe {
        lua_createtable(state, 0, 0);
        Table::from_ref(lua.pop_ref())
    }
}
