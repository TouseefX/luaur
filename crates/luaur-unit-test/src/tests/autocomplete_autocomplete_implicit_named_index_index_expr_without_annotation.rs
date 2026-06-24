//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4676:autocomplete_autocomplete_implicit_named_index_index_expr_without_annotation`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_implicit_named_index_index_expr_without_annotation

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_implicit_named_index_index_expr_without_annotation() {
    use crate::records::ac_fixture::AcFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        local foo = {
            ["Item/Foo"] = 42,
            ["Item/Bar"] = "it's true",
            ["Item/Baz"] = true,
        }
        foo["@1"]
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    for (key, expected_type) in [
        ("Item/Foo", "number"),
        ("Item/Bar", "string"),
        ("Item/Baz", "boolean"),
    ] {
        assert_eq!(ac.entry_map.contains_key(key), true);
        let entry = &ac.entry_map[key];
        assert_eq!(entry.kind, AutocompleteEntryKind::Property);
        let ty = entry.r#type.expect("autocomplete entry should have a type");
        assert_eq!(expected_type, to_string_type_id(ty));
    }
}
