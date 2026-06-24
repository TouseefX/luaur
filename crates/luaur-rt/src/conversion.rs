//! `FromLua` / `IntoLua` / `FromLuaMulti` / `IntoLuaMulti` impls for the common
//! Rust types. Mirrors the impls in `mlua::conversion`.

use crate::error::{Error, Result};
use crate::function::Function;
use crate::multi::{MultiValue, Variadic};
use crate::state::Lua;
use crate::string::LuaString;
use crate::table::Table;
use crate::traits::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti};
use crate::value::{Integer, Number, Value};

// ---------------------------------------------------------------------------
// Value itself
// ---------------------------------------------------------------------------

impl IntoLua for Value {
    fn into_lua(self, _lua: &Lua) -> Result<Value> {
        Ok(self)
    }
}

impl FromLua for Value {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        Ok(value)
    }
}

// ---------------------------------------------------------------------------
// Unit / nil
// ---------------------------------------------------------------------------
//
// NOTE: `()` is deliberately NOT a single-value (`IntoLua`/`FromLua`) type.
// In Lua, `()` means *zero* values, not one nil — so it implements only the
// multi-value traits below (producing/consuming no stack values). This also
// avoids a coherence clash with the blanket `impl<T: IntoLua> IntoLuaMulti`.

// `()` as a *multi* value means "no values" in both directions.
impl IntoLuaMulti for () {
    fn into_lua_multi(self, _lua: &Lua) -> Result<MultiValue> {
        Ok(MultiValue::new())
    }
}

impl FromLuaMulti for () {
    fn from_lua_multi(_values: MultiValue, _lua: &Lua) -> Result<Self> {
        Ok(())
    }
}

// ---------------------------------------------------------------------------
// bool
// ---------------------------------------------------------------------------

impl IntoLua for bool {
    fn into_lua(self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Boolean(self))
    }
}

impl FromLua for bool {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        // Lua truthiness: nil and false are false, everything else is true.
        Ok(match value {
            Value::Nil => false,
            Value::Boolean(b) => b,
            _ => true,
        })
    }
}

// ---------------------------------------------------------------------------
// Integers (range-checked)
// ---------------------------------------------------------------------------

macro_rules! impl_integer {
    ($($ty:ty),*) => {$(
        impl IntoLua for $ty {
            fn into_lua(self, _lua: &Lua) -> Result<Value> {
                let as_i64 = i64::try_from(self).map_err(|_| Error::ToLuaConversionError {
                    from: stringify!($ty),
                    to: "integer",
                    message: Some("value out of i64 range".to_string()),
                })?;
                Ok(Value::Integer(as_i64))
            }
        }

        impl FromLua for $ty {
            fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
                let n: i64 = match value {
                    Value::Integer(i) => i,
                    Value::Number(f) => {
                        if f.fract() != 0.0 || !f.is_finite() {
                            return Err(Error::FromLuaConversionError {
                                from: "number",
                                to: stringify!($ty).to_string(),
                                message: Some("number has no integer representation".to_string()),
                            });
                        }
                        f as i64
                    }
                    other => {
                        return Err(Error::FromLuaConversionError {
                            from: other.type_name(),
                            to: stringify!($ty).to_string(),
                            message: None,
                        });
                    }
                };
                <$ty>::try_from(n).map_err(|_| Error::FromLuaConversionError {
                    from: "number",
                    to: stringify!($ty).to_string(),
                    message: Some("value out of target range".to_string()),
                })
            }
        }
    )*};
}

impl_integer!(i8, u8, i16, u16, i32, u32, i64, u64, isize, usize);

// `Integer` is `i64`, already covered by impl_integer.
const _: () = {
    // Compile-time assertion that Integer == i64.
    fn _assert(_x: Integer) -> i64 {
        _x
    }
};

// ---------------------------------------------------------------------------
// Floats
// ---------------------------------------------------------------------------

impl IntoLua for f64 {
    fn into_lua(self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Number(self))
    }
}

impl FromLua for f64 {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::Number(n) => Ok(n),
            Value::Integer(i) => Ok(i as f64),
            other => Err(Error::FromLuaConversionError {
                from: other.type_name(),
                to: "f64".to_string(),
                message: None,
            }),
        }
    }
}

impl IntoLua for f32 {
    fn into_lua(self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Number(self as Number))
    }
}

impl FromLua for f32 {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        Ok(f64::from_lua(value, lua)? as f32)
    }
}

// ---------------------------------------------------------------------------
// Strings
// ---------------------------------------------------------------------------

impl IntoLua for String {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        Ok(Value::String(lua.create_string(&self)))
    }
}

impl IntoLua for &str {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        Ok(Value::String(lua.create_string(self)))
    }
}

impl IntoLua for &String {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        Ok(Value::String(lua.create_string(self)))
    }
}

impl FromLua for String {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::String(s) => s.to_str(),
            // Lua coerces numbers to strings in many contexts; mirror that.
            Value::Integer(i) => Ok(i.to_string()),
            Value::Number(n) => Ok(n.to_string()),
            other => Err(Error::FromLuaConversionError {
                from: other.type_name(),
                to: "String".to_string(),
                message: None,
            }),
        }
    }
}

impl IntoLua for LuaString {
    fn into_lua(self, _lua: &Lua) -> Result<Value> {
        Ok(Value::String(self))
    }
}

impl FromLua for LuaString {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::String(s) => Ok(s),
            Value::Integer(i) => Ok(_lua.create_string(i.to_string())),
            Value::Number(n) => Ok(_lua.create_string(n.to_string())),
            other => Err(Error::FromLuaConversionError {
                from: other.type_name(),
                to: "String".to_string(),
                message: None,
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Handles (Table, Function)
// ---------------------------------------------------------------------------

impl IntoLua for Table {
    fn into_lua(self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Table(self))
    }
}

impl FromLua for Table {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::Table(t) => Ok(t),
            other => Err(Error::FromLuaConversionError {
                from: other.type_name(),
                to: "Table".to_string(),
                message: None,
            }),
        }
    }
}

impl IntoLua for Function {
    fn into_lua(self, _lua: &Lua) -> Result<Value> {
        Ok(Value::Function(self))
    }
}

impl FromLua for Function {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::Function(f) => Ok(f),
            other => Err(Error::FromLuaConversionError {
                from: other.type_name(),
                to: "Function".to_string(),
                message: None,
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Option<T>
// ---------------------------------------------------------------------------

impl<T: IntoLua> IntoLua for Option<T> {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        match self {
            Some(v) => v.into_lua(lua),
            None => Ok(Value::Nil),
        }
    }
}

impl<T: FromLua> FromLua for Option<T> {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        match value {
            Value::Nil => Ok(None),
            other => Ok(Some(T::from_lua(other, lua)?)),
        }
    }
}

// ---------------------------------------------------------------------------
// Vec<T> <-> sequence table
// ---------------------------------------------------------------------------

impl<T: IntoLua> IntoLua for Vec<T> {
    fn into_lua(self, lua: &Lua) -> Result<Value> {
        let table = lua.create_table();
        for (i, item) in self.into_iter().enumerate() {
            // Lua sequences are 1-based.
            table.set((i + 1) as i64, item)?;
        }
        Ok(Value::Table(table))
    }
}

impl<T: FromLua> FromLua for Vec<T> {
    fn from_lua(value: Value, _lua: &Lua) -> Result<Self> {
        match value {
            Value::Table(t) => {
                let len = t.raw_len();
                let mut out = Vec::with_capacity(len);
                for i in 1..=len {
                    out.push(t.get::<i64, T>(i as i64)?);
                }
                Ok(out)
            }
            other => Err(Error::FromLuaConversionError {
                from: other.type_name(),
                to: "Vec".to_string(),
                message: None,
            }),
        }
    }
}

// ---------------------------------------------------------------------------
// Variadic<T>
// ---------------------------------------------------------------------------

impl<T: IntoLua> IntoLuaMulti for Variadic<T> {
    fn into_lua_multi(self, lua: &Lua) -> Result<MultiValue> {
        let vec: Vec<T> = self.into();
        let mut m = MultiValue::with_capacity(vec.len());
        for item in vec {
            m.push_back(item.into_lua(lua)?);
        }
        Ok(m)
    }
}

impl<T: FromLua> FromLuaMulti for Variadic<T> {
    fn from_lua_multi(values: MultiValue, lua: &Lua) -> Result<Self> {
        let mut out = Vec::with_capacity(values.len());
        for v in values {
            out.push(T::from_lua(v, lua)?);
        }
        Ok(Variadic::from(out))
    }
}

// ---------------------------------------------------------------------------
// MultiValue passthrough
// ---------------------------------------------------------------------------

impl IntoLuaMulti for MultiValue {
    fn into_lua_multi(self, _lua: &Lua) -> Result<MultiValue> {
        Ok(self)
    }
}

impl FromLuaMulti for MultiValue {
    fn from_lua_multi(values: MultiValue, _lua: &Lua) -> Result<Self> {
        Ok(values)
    }
}

// ---------------------------------------------------------------------------
// Tuples (IntoLuaMulti / FromLuaMulti) up to 12
// ---------------------------------------------------------------------------

macro_rules! impl_tuple {
    () => {};
    ($first:ident $($rest:ident)*) => {
        impl_tuple!($($rest)*);

        #[allow(non_snake_case)]
        impl<$first: IntoLua, $($rest: IntoLua,)*> IntoLuaMulti for ($first, $($rest,)*) {
            fn into_lua_multi(self, lua: &Lua) -> Result<MultiValue> {
                let ($first, $($rest,)*) = self;
                let mut m = MultiValue::new();
                m.push_back($first.into_lua(lua)?);
                $( m.push_back($rest.into_lua(lua)?); )*
                Ok(m)
            }
        }

        #[allow(non_snake_case)]
        impl<$first: FromLua, $($rest: FromLua,)*> FromLuaMulti for ($first, $($rest,)*) {
            fn from_lua_multi(mut values: MultiValue, lua: &Lua) -> Result<Self> {
                let $first = $first::from_lua(values.pop_front().unwrap_or(Value::Nil), lua)?;
                $( let $rest = $rest::from_lua(values.pop_front().unwrap_or(Value::Nil), lua)?; )*
                Ok(($first, $($rest,)*))
            }
        }
    };
}

impl_tuple!(A B C D E F G H I J K L);
