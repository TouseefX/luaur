//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2197:autocomplete_type_correct_sealed_table`
//! Source: `tests/Autocomplete.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Autocomplete.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Autocomplete.h
//!   - includes -> source_file Analysis/include/Luau/AutocompleteTypes.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Common/include/Luau/StringUtils.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Autocomplete.test.cpp
//! - outgoing:
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - translates_to -> rust_item autocomplete_type_correct_sealed_table

#[cfg(test)]
#[test]
fn autocomplete_type_correct_sealed_table() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::solver_mode::SolverMode;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local function f(a: { x: number, y: number }) return a.x + a.y end
local fp: @1= f
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    // The skeleton fixture currently runs the translated old solver path.
    if !luaur_common::FFlag::DebugLuauForceOldSolver.get()
        && fixture.base.get_frontend().get_luau_solver_mode() == SolverMode::New
    {
        assert_eq!(
            "({ x: number, y: number }) -> number",
            to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
        );
    } else {
        assert_eq!(
            "({ x: number, y: number }) -> (...any)",
            to_string_type_id(fixture.base.base.require_type_string(&String::from("f")))
        );
    }

    assert!(ac
        .entry_map
        .contains_key("({ x: number, y: number }) -> number"));
}
