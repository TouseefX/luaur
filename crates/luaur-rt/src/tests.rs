//! Integration-style unit tests exercising the mlua-style API end to end.

use crate::prelude::*;
use std::cell::Cell;
use std::rc::Rc;

#[test]
fn create_function_and_call_from_lua() {
    let lua = Lua::new();
    let add = lua
        .create_function(|_, (a, b): (i64, i64)| Ok(a + b))
        .unwrap();
    lua.globals().set("add", add).unwrap();
    let result: i64 = lua.load("return add(2, 3)").eval().unwrap();
    assert_eq!(result, 5);
}

#[test]
fn capturing_closure_counter() {
    let lua = Lua::new();
    let counter = Rc::new(Cell::new(0i64));
    let c2 = counter.clone();
    let inc = lua
        .create_function(move |_, ()| {
            c2.set(c2.get() + 1);
            Ok(c2.get())
        })
        .unwrap();
    lua.globals().set("inc", inc).unwrap();
    lua.load("inc(); inc(); inc()").exec().unwrap();
    assert_eq!(counter.get(), 3);
    let last: i64 = lua.load("return inc()").eval().unwrap();
    assert_eq!(last, 4);
    assert_eq!(counter.get(), 4);
}

#[test]
fn scalar_string_bool_option_round_trips() {
    let lua = Lua::new();

    let i: i64 = lua.load("return 42").eval().unwrap();
    assert_eq!(i, 42);

    let f: f64 = lua.load("return 3.5").eval().unwrap();
    assert_eq!(f, 3.5);

    let s: String = lua.load("return 'hello'").eval().unwrap();
    assert_eq!(s, "hello");

    let b: bool = lua.load("return true").eval().unwrap();
    assert!(b);

    let none: Option<i64> = lua.load("return nil").eval().unwrap();
    assert_eq!(none, None);

    let some: Option<i64> = lua.load("return 7").eval().unwrap();
    assert_eq!(some, Some(7));

    // Round-trip through a Rust function: identity on each type.
    let id_i = lua.create_function(|_, x: i64| Ok(x)).unwrap();
    lua.globals().set("id_i", id_i).unwrap();
    let back: i64 = lua.load("return id_i(123)").eval().unwrap();
    assert_eq!(back, 123);

    let id_s = lua.create_function(|_, x: String| Ok(x)).unwrap();
    lua.globals().set("id_s", id_s).unwrap();
    let back_s: String = lua.load("return id_s('world')").eval().unwrap();
    assert_eq!(back_s, "world");
}

#[test]
fn table_set_get_nested() {
    let lua = Lua::new();
    let t = lua.create_table();
    t.set("a", 1i64).unwrap();
    t.set("b", "two").unwrap();
    let inner = lua.create_table();
    inner.set("x", 10i64).unwrap();
    t.set("inner", inner).unwrap();

    let a: i64 = t.get("a").unwrap();
    assert_eq!(a, 1);
    let b: String = t.get("b").unwrap();
    assert_eq!(b, "two");
    assert!(t.contains_key("a").unwrap());
    assert!(!t.contains_key("missing").unwrap());

    let inner: Table = t.get("inner").unwrap();
    let x: i64 = inner.get("x").unwrap();
    assert_eq!(x, 10);
}

#[test]
fn vec_to_and_from_table() {
    let lua = Lua::new();
    // Vec -> table -> Lua, summed in Lua, back to Rust.
    let v = vec![1i64, 2, 3, 4];
    lua.globals().set("nums", v.clone()).unwrap();
    let len: i64 = lua.load("return #nums").eval().unwrap();
    assert_eq!(len, 4);
    let sum: i64 = lua
        .load("local s = 0; for _, n in ipairs(nums) do s = s + n end; return s")
        .eval()
        .unwrap();
    assert_eq!(sum, 10);

    // Lua sequence -> Rust Vec.
    let back: Vec<i64> = lua.load("return {10, 20, 30}").eval().unwrap();
    assert_eq!(back, vec![10, 20, 30]);
}

#[test]
fn table_pairs_iteration() {
    let lua = Lua::new();
    let t = lua.create_table();
    t.set("a", 1i64).unwrap();
    t.set("b", 2i64).unwrap();
    t.set("c", 3i64).unwrap();
    let mut sum = 0i64;
    let mut keys = Vec::new();
    for pair in t.pairs::<String, i64>() {
        let (k, v) = pair.unwrap();
        keys.push(k);
        sum += v;
    }
    keys.sort();
    assert_eq!(keys, vec!["a", "b", "c"]);
    assert_eq!(sum, 6);
}

#[test]
fn call_lua_function_from_rust() {
    let lua = Lua::new();
    let doubler: Function = lua.load("return function(x) return x * 2 end").eval().unwrap();
    let r: i64 = doubler.call(21i64).unwrap();
    assert_eq!(r, 42);
}

#[test]
fn rust_error_is_catchable_by_lua_pcall() {
    let lua = Lua::new();
    let boom = lua
        .create_function(|_, ()| -> Result<()> { Err(Error::runtime("kaboom")) })
        .unwrap();
    lua.globals().set("boom", boom).unwrap();
    // pcall should catch the error and report ok=false plus the message.
    let msg: String = lua
        .load("local ok, err = pcall(boom); assert(not ok); return tostring(err)")
        .eval()
        .unwrap();
    assert!(msg.contains("kaboom"), "expected message to contain kaboom: {msg}");
}

#[test]
fn lua_error_call_surfaces_as_err() {
    let lua = Lua::new();
    let err = lua.load("error('boom')").exec().unwrap_err();
    let text = err.to_string();
    assert!(text.contains("boom"), "error should mention boom: {text}");
}

#[test]
fn rust_panic_in_closure_becomes_lua_error_not_abort() {
    let lua = Lua::new();
    let panicky = lua
        .create_function(|_, ()| -> Result<()> {
            panic!("intentional panic from rust callback");
        })
        .unwrap();
    lua.globals().set("panicky", panicky).unwrap();
    // The panic must be caught and converted to a catchable Lua error — the
    // process must NOT abort.
    let caught: String = lua
        .load("local ok, err = pcall(panicky); assert(not ok); return tostring(err)")
        .eval()
        .unwrap();
    assert!(
        caught.contains("intentional panic"),
        "panic message should surface: {caught}"
    );

    // And calling it directly (no pcall) surfaces as Err on the Rust side.
    let direct = lua.load("panicky()").exec();
    assert!(direct.is_err());
}

// --- UserData ------------------------------------------------------------

struct Vec2 {
    x: f64,
    y: f64,
}

impl UserData for Vec2 {
    fn add_methods<M: UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("magnitude", |_, this, ()| {
            Ok((this.x * this.x + this.y * this.y).sqrt())
        });
        methods.add_method("get_x", |_, this, ()| Ok(this.x));
        methods.add_method_mut("scale", |_, this, factor: f64| {
            this.x *= factor;
            this.y *= factor;
            Ok(())
        });
        // __add meta-method: returns the component-wise sum's magnitude (we
        // can't easily return a fresh Vec2 userdata from here without a
        // constructor, so we return a number to keep the test self-contained).
        methods.add_meta_method("__add", |_, this, other: f64| Ok(this.x + this.y + other));
    }
}

#[test]
fn userdata_method_from_lua() {
    let lua = Lua::new();
    let v = lua.create_userdata(Vec2 { x: 3.0, y: 4.0 }).unwrap();
    lua.globals().set("v", v).unwrap();

    let mag: f64 = lua.load("return v:magnitude()").eval().unwrap();
    assert!((mag - 5.0).abs() < 1e-9, "magnitude should be 5, got {mag}");

    let x: f64 = lua.load("return v:get_x()").eval().unwrap();
    assert_eq!(x, 3.0);
}

#[test]
fn userdata_method_mut_from_lua() {
    let lua = Lua::new();
    let v = lua.create_userdata(Vec2 { x: 3.0, y: 4.0 }).unwrap();
    lua.globals().set("v", v).unwrap();
    let mag: f64 = lua.load("v:scale(2); return v:magnitude()").eval().unwrap();
    assert!((mag - 10.0).abs() < 1e-9, "scaled magnitude should be 10, got {mag}");
}

#[test]
fn userdata_meta_method_add() {
    let lua = Lua::new();
    let v = lua.create_userdata(Vec2 { x: 3.0, y: 4.0 }).unwrap();
    lua.globals().set("v", v).unwrap();
    // v + 10 -> __add(v, 10) -> 3 + 4 + 10 = 17
    let r: f64 = lua.load("return v + 10").eval().unwrap();
    assert_eq!(r, 17.0);
}

#[test]
fn variadic_args() {
    let lua = Lua::new();
    let sum = lua
        .create_function(|_, nums: Variadic<i64>| Ok(nums.iter().sum::<i64>()))
        .unwrap();
    lua.globals().set("vsum", sum).unwrap();
    let r: i64 = lua.load("return vsum(1, 2, 3, 4, 5)").eval().unwrap();
    assert_eq!(r, 15);
}

#[test]
fn multiple_return_values_via_tuple() {
    let lua = Lua::new();
    let (a, b, c): (i64, String, bool) = lua.load("return 1, 'two', true").eval().unwrap();
    assert_eq!(a, 1);
    assert_eq!(b, "two");
    assert!(c);
}

#[test]
fn integer_range_check_rejects_overflow() {
    let lua = Lua::new();
    // u8 from a number larger than 255 should fail conversion.
    let id_u8 = lua.create_function(|_, x: u8| Ok(x)).unwrap();
    lua.globals().set("id_u8", id_u8).unwrap();
    let err = lua.load("return id_u8(300)").exec();
    assert!(err.is_err(), "u8 conversion from 300 must error");
}
