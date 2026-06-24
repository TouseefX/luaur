//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:521:autocomplete_method_call_inside_if_conditional`
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
//!   - calls -> method TxnLog::concat (Analysis/src/TxnLog.cpp)
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_method_call_inside_if_conditional

#[cfg(test)]
#[test]
fn autocomplete_method_call_inside_if_conditional() {
    use crate::records::ac_builtins_fixture::ACBuiltinsFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = ACBuiltinsFixture::default();
    fixture.base.check(&String::from(
        r#"
        if table:  @1
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert_ne!(0, ac.entry_map.len());
    assert!(ac.entry_map.contains_key("concat"));
    assert!(!ac.entry_map.contains_key("math"));
    assert_eq!(ac.context, AutocompleteContext::Property);
}
