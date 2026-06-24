//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4507:autocomplete_ac_dont_overflow_on_recursive_union`
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
//!   - calls -> method MagicInstanceIsA::infer (tests/TypeInfer.refinements.test.cpp)
//!   - translates_to -> rust_item autocomplete_ac_dont_overflow_on_recursive_union

#[cfg(test)]
#[test]
fn autocomplete_ac_dont_overflow_on_recursive_union() {
    use crate::functions::register_ac_extern_type_fixture_types::register_ac_extern_type_fixture_types;
    use crate::records::ac_extern_type_fixture::AcExternTypeFixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let mut fixture = AcExternTypeFixture::default();
    register_ac_extern_type_fixture_types(&mut fixture.base);

    fixture.base.check(&String::from(
        r#"
        local table1: {ChildClass} = {}
        local table2 = {}

        for index, value in table2[1] do
            table.insert(table1, value)
            value.@1
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert!(
            ac.entry_map.contains_key("BaseMethod"),
            "entries: {:?}",
            ac.entry_map.keys().collect::<alloc::vec::Vec<_>>()
        );
        assert!(
            ac.entry_map.contains_key("Method"),
            "entries: {:?}",
            ac.entry_map.keys().collect::<alloc::vec::Vec<_>>()
        );
    } else {
        assert!(ac.entry_map.is_empty());
    }
}
