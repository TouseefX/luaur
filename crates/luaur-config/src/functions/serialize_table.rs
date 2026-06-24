use alloc::string::String;
use core::ffi::CStr;

use crate::records::config_table::ConfigTable;
use crate::records::config_table_key::ConfigTableKey;
use crate::records::config_value::ConfigValue;
use crate::records::thread_popper::ThreadPopper;
use luaur_vm::enums::lua_type::lua_Type;
use luaur_vm::functions::lua_next::lua_next;
use luaur_vm::functions::lua_pushnil::lua_pushnil;
use luaur_vm::functions::lua_pushvalue::lua_pushvalue;
use luaur_vm::functions::lua_toboolean::lua_toboolean;
use luaur_vm::functions::lua_tolstring::lua_tolstring;
use luaur_vm::functions::lua_tonumberx::lua_tonumberx;
use luaur_vm::functions::lua_type::lua_type;
use luaur_vm::type_aliases::lua_state::lua_State;

pub fn serialize_table(l: *mut lua_State, error: &mut String) -> Option<ConfigTable> {
    let _table_popper = ThreadPopper::thread_popper_lua_state(l);
    let mut table = ConfigTable::new();

    unsafe {
        lua_pushnil(l);
        while lua_next(l, -2) != 0 {
            let _value_popper = ThreadPopper::thread_popper_lua_state(l);

            let key = match lua_type(l, -2) {
                t if t == lua_Type::LUA_TNUMBER as i32 => {
                    ConfigTableKey::from(lua_tonumberx(l, -2, core::ptr::null_mut()))
                }
                t if t == lua_Type::LUA_TSTRING as i32 => ConfigTableKey::from(lua_string(l, -2)),
                _ => {
                    *error = String::from("configuration table keys must be strings or numbers");
                    return None;
                }
            };

            match lua_type(l, -1) {
                t if t == lua_Type::LUA_TNUMBER as i32 => {
                    table.insert(
                        key,
                        ConfigValue::from(lua_tonumberx(l, -1, core::ptr::null_mut())),
                    );
                }
                t if t == lua_Type::LUA_TSTRING as i32 => {
                    table.insert(key, ConfigValue::from(lua_string(l, -1)));
                }
                t if t == lua_Type::LUA_TBOOLEAN as i32 => {
                    table.insert(key, ConfigValue::from(lua_toboolean(l, -1) != 0));
                }
                t if t == lua_Type::LUA_TTABLE as i32 => {
                    lua_pushvalue(l, -1);
                    if let Some(nested) = serialize_table(l, error) {
                        table.insert(key, ConfigValue::from(nested));
                    } else {
                        return None;
                    }
                }
                _ => {
                    *error = alloc::format!(
                        "configuration value for key \"{}\" must be a string, number, boolean, or nested table",
                        key.to_string()
                    );
                    return None;
                }
            }
        }
    }

    Some(table)
}

unsafe fn lua_string(l: *mut lua_State, index: i32) -> String {
    let ptr = lua_tolstring(l, index, core::ptr::null_mut());
    if ptr.is_null() {
        String::new()
    } else {
        CStr::from_ptr(ptr).to_string_lossy().into_owned()
    }
}
