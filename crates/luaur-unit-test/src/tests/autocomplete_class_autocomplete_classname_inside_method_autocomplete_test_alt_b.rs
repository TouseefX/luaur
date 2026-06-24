//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:5347:autocomplete_class_autocomplete_classname_inside_method`
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
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - translates_to -> rust_item autocomplete_class_autocomplete_classname_inside_method

#[cfg(test)]
#[test]
fn autocomplete_class_autocomplete_classname_inside_method() {
    use crate::records::ac_fixture::AcFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _classes = ScopedFastFlag::new(&FFlag::DebugLuauUserDefinedClasses, true);

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        class Bar
            public value: number
            function new()
                return B@1
            end
        end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac.entry_map.contains_key("Bar"));
}
