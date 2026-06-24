//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Config.test.cpp:327:config_interrupt_execution`
//! Source: `tests/Config.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Config.test.cpp
//! - source_includes:
//!   - includes -> source_file Config/include/Luau/Config.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Config/include/Luau/LinterConfig.h
//!   - includes -> source_file Config/include/Luau/LuauConfig.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Config.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record ConfigTable (Config/include/Luau/LuauConfig.h)
//!   - calls -> function extractConfig (Config/src/LuauConfig.cpp)
//!   - translates_to -> rust_item config_interrupt_execution

#[cfg(test)]
#[test]
fn config_interrupt_execution() {
    use luaur_config::functions::extract_config::extract_config;
    use luaur_config::records::interrupt_callbacks::InterruptCallbacks;
    use luaur_vm::enums::lua_status::lua_Status;
    use luaur_vm::functions::lua_g_pusherror::lua_g_pusherror;
    use luaur_vm::type_aliases::lua_state::lua_State;

    unsafe extern "C-unwind" fn interrupt(l: *mut lua_State, _gc: core::ffi::c_int) {
        unsafe {
            lua_g_pusherror(l, c"interrupted".as_ptr());
            (*l).status = lua_Status::LUA_ERRRUN as u8;
        }
    }

    let source = r#"
        while true do end
    "#
    .to_string();

    let mut error = String::new();
    let config_table = extract_config(
        &source,
        &InterruptCallbacks {
            init_callback: None,
            interrupt_callback: Some(interrupt),
        },
        &mut error,
    );
    assert!(config_table.is_none());
    assert!(error.contains("interrupted"), "{error}");
}
