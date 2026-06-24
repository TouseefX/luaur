//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:4652:autocomplete_autocomplete_implicit_named_index_index_expr`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Constraint (Analysis/include/Luau/Constraint.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_autocomplete_implicit_named_index_index_expr

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_implicit_named_index_index_expr() {
    use crate::records::ac_fixture::AcFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        type Constraint = "A" | "B" | "C"
        local foo : { [Constraint]: string } = {
            A = "Value for A",
            B = "Value for B",
            C = "Value for C",
        }
        foo["@1"]
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(ac.entry_map.contains_key("A"), true);
    assert_eq!(ac.entry_map["A"].kind, AutocompleteEntryKind::String);
    assert_eq!(ac.entry_map.contains_key("B"), true);
    assert_eq!(ac.entry_map["B"].kind, AutocompleteEntryKind::String);
    assert_eq!(ac.entry_map.contains_key("C"), true);
    assert_eq!(ac.entry_map["C"].kind, AutocompleteEntryKind::String);
}
