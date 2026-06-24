//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5261:autocomplete_cli_197197_autocomplete_generic_keyof`
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
//!   - type_ref -> type_alias ScopedFastFlag (tests/ScopedFlags.h)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item autocomplete_cli_197197_autocomplete_generic_keyof

#[cfg(test)]
#[test]
fn autocomplete_cli_197197_autocomplete_generic_keyof() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(&String::from(
        r#"
        local function ToggleButton<T>(Table: T, Key: keyof<T>)
            -- don't need to do anything here.
        end

        local tbl: { Changed: bool, RemoveTag: bool } = nil :: any

        ToggleButton(tbl, "@1")
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("Changed"));
    assert!(ac.entry_map.contains_key("RemoveTag"));
}
