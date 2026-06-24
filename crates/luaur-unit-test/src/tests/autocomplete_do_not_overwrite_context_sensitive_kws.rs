//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:591:autocomplete_do_not_overwrite_context_sensitive_kws`
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
//!   - type_ref -> record AutocompleteEntry (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> enum AutocompleteEntryKind (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
//!   - translates_to -> rust_item autocomplete_do_not_overwrite_context_sensitive_kws

#[cfg(test)]
#[test]
fn autocomplete_do_not_overwrite_context_sensitive_kws() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;
    use luaur_analysis::enums::autocomplete_entry_kind::AutocompleteEntryKind;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        local function continue()
        end


@1    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    let entry = &ac.entry_map["continue"];

    assert_eq!(entry.kind, AutocompleteEntryKind::Binding);
    assert_eq!(ac.context, AutocompleteContext::Statement);
}
