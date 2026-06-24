// Adapted from mlua (https://github.com/mlua-rs/mlua), MIT License,
// © 2019 Aleksandr Orlenko / mlua authors. See tests/ATTRIBUTION.md.
//
// Only the Luau-relevant, non-deferred subset of mlua's `tests/luau.rs` is
// ported.
//
// Phase 2 added the Luau-specific runtime types: the **vector** tests below,
// and **buffer** coverage in its own file (`tests/mlua_buffer.rs`, ported
// verbatim from mlua's `tests/buffer.rs`).
//
// Still dropped (each needs luaur-rt API surface from a later phase):
//   - `test_vectors` *fastcall* half, `test_vector_metatable`
//                                  -> need `Compiler` / `Chunk::set_compiler` /
//                                     `set_vector_ctor`/`set_vector_type` /
//                                     `Lua::set_type_metatable`.
//   - sandbox / sandbox_safeenv / sandbox_nolibs / sandbox_threads
//                                  -> need `Lua::sandbox` / `Thread::sandbox` /
//                                     `Globals::set_safeenv`.
//   - interrupts                   -> need `set_interrupt` / `VmState`.
//   - fflags                       -> need `Lua::set_fflag`.
//   - memory categories, heap dumps-> need the memory-category / heap-dump API.
//   - integer64 type               -> needs `LuauIntegerType2` + `42i` literals.
//   - typeof(error-value)          -> luaur-rt carries `Value::Error` as a
//                                     string, so `typeof` reports "string".
//   - loadstring                   -> not registered by luaur's base library
//                                     (see `test_load_from_rust` for the analog).
//   - `mod require`                -> the `require` submodule (path resolution).

use luaur_rt::{Error, Lua, Result, Table, Value, Vector};

#[test]
fn test_version() -> Result<()> {
    let lua = Lua::new();
    // DEVIATION: luaur's VM reports `_VERSION` as the bare string `"Luau"` (no
    // trailing ` 0.<minor>` like upstream/mlua's bundled Luau), so this checks
    // the `"Luau"` prefix rather than `"Luau 0."`. The API surface exercised
    // (`globals().get::<String>`) is unchanged.
    assert!(lua.globals().get::<String>("_VERSION")?.starts_with("Luau"));
    Ok(())
}

#[test]
fn test_vectors() -> Result<()> {
    let lua = Lua::new();

    let v: Vector = lua
        .load("vector.create(1, 2, 3) + vector.create(3, 2, 1)")
        .eval()?;
    assert_eq!(v, [4.0, 4.0, 4.0]);

    // Test conversion into Rust array
    let v: [f64; 3] = lua.load("vector.create(1, 2, 3)").eval()?;
    assert!(v == [1.0, 2.0, 3.0]);

    // Test vector methods
    lua.load(
        r#"
        local v = vector.create(1, 2, 3)
        assert(v.x == 1)
        assert(v.y == 2)
        assert(v.z == 3)
    "#,
    )
    .exec()?;

    // SKIPPED (later phase): mlua's `test_vectors` second half drives a
    // `Compiler::new().set_vector_ctor("vector")` fastcall path. `Compiler` /
    // `Chunk::set_compiler` are not yet part of luaur-rt's surface.

    Ok(())
}

#[test]
fn test_vector_roundtrip() -> Result<()> {
    // Additional Luau-vector coverage (not in mlua's luau.rs): round-trip a
    // `Vector` built on the Rust side through Lua and back, exercising
    // `Lua::create_vector`, the component accessors, `IntoLua`/`FromLua`, and
    // `Value::Vector` equality in both directions.
    let lua = Lua::new();

    let v = lua.create_vector(1.5, -2.0, 3.25);
    assert_eq!(v.x(), 1.5);
    assert_eq!(v.y(), -2.0);
    assert_eq!(v.z(), 3.25);

    let echo = lua.create_function(|_, v: Vector| Ok(v))?;
    let back: Vector = echo.call(v)?;
    assert_eq!(back, v);
    assert_eq!(back, [1.5, -2.0, 3.25]);

    // A vector passed in from Rust is observable as the Luau `vector` type.
    let described: String = lua
        .create_function(|_, v: Vector| Ok(format!("{},{},{}", v.x(), v.y(), v.z())))?
        .call(lua.create_vector(4.0, 5.0, 6.0))?;
    assert_eq!(described, "4,5,6");

    Ok(())
}

#[test]
fn test_load_from_rust() -> Result<()> {
    // DEVIATION: mlua's `test_loadstring` exercises the Lua-level `loadstring`
    // builtin, which luaur's base library does not register. The luaur-rt
    // analog is `Lua::load(...).into_function()`, which compiles a string into a
    // callable function — exercised here with the same observable result.
    let lua = Lua::new();

    let f = lua.load("return 123").into_function()?;
    assert_eq!(f.call::<i32>(())?, 123);

    Ok(())
}

#[test]
fn test_readonly_table() -> Result<()> {
    let lua = Lua::new();

    let t = lua.create_sequence_from([1])?;
    assert!(!t.is_readonly());
    t.set_readonly(true);
    assert!(t.is_readonly());

    fn check_readonly_error<T: std::fmt::Debug>(res: Result<T>) {
        match res {
            Err(Error::RuntimeError(e)) if e.contains("attempt to modify a readonly table") => {}
            r => panic!("expected readonly RuntimeError, got {r:?}"),
        }
    }

    check_readonly_error(t.set("key", "value"));
    check_readonly_error(t.raw_set("key", "value"));
    check_readonly_error(t.raw_insert(1, "value"));
    check_readonly_error(t.raw_remove(1));
    check_readonly_error(t.push("value"));
    check_readonly_error(t.pop::<Value>());
    check_readonly_error(t.raw_push("value"));
    check_readonly_error(t.raw_pop::<Value>());

    // Special case: cannot change the metatable of a readonly table.
    check_readonly_error(t.set_metatable(None));

    // Flipping back to writable restores mutation.
    t.set_readonly(false);
    t.set("key", "value")?;
    assert_eq!(t.get::<String>("key")?, "value");

    Ok(())
}

#[test]
fn test_readonly_table_reads_still_work() -> Result<()> {
    let lua = Lua::new();

    let t = lua.create_sequence_from([10, 20, 30])?;
    t.set_readonly(true);
    // Reads must remain available on a readonly table.
    assert_eq!(t.get::<i64>(1)?, 10);
    assert_eq!(t.raw_get::<i64>(2)?, 20);
    assert_eq!(t.raw_len(), 3);
    assert_eq!(t.sequence_values::<i64>().collect::<Result<Vec<_>>>()?, vec![10, 20, 30]);

    Ok(())
}

#[test]
fn test_metatable_via_lua() -> Result<()> {
    // A small Luau metatable round-trip (set_metatable + __index function),
    // mirroring the spirit of mlua's vector-metatable test without vectors.
    let lua = Lua::new();

    let base = lua
        .load(
            r#"
            return {
                __index = {
                    greet = function() return "hi" end,
                }
            }
        "#,
        )
        .eval::<Table>()?;

    let t = lua.create_table();
    t.set_metatable(Some(base))?;
    let greeting: String = lua
        .load("local t = ...; return t.greet()")
        .into_function()?
        .call(t)?;
    assert_eq!(greeting, "hi");

    Ok(())
}
