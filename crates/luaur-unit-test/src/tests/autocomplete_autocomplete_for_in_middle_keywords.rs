//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:734:autocomplete_autocomplete_for_in_middle_keywords`
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
//!   - type_ref -> enum AutocompleteContext (Analysis/include/Luau/AutocompleteTypes.h)
//!   - type_ref -> record Statement (Analysis/src/Linter.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_for_in_middle_keywords

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_for_in_middle_keywords() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        for @1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac1.entry_map.len());
    assert_eq!(ac1.context, AutocompleteContext::Unknown);

    fixture.base.check(&String::from(
        r#"
        for x@1 @2
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac2.entry_map.len());
    assert_eq!(ac2.context, AutocompleteContext::Unknown);

    let ac2a = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert_eq!(1, ac2a.entry_map.len());
    assert_eq!(1, ac2a.entry_map.get("in").map_or(0, |_| 1));
    assert_eq!(ac2a.context, AutocompleteContext::Keyword);

    fixture.base.check(&String::from(
        r#"
        for x in y@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac3.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(0, ac3.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac3.context, AutocompleteContext::Expression);

    fixture.base.check(&String::from(
        r#"
        for x in y @1
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac4.entry_map.len());
    assert_eq!(1, ac4.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac4.context, AutocompleteContext::Keyword);

    fixture.base.check(&String::from(
        r#"
        for x in f f@1
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac5.entry_map.len());
    assert_eq!(1, ac5.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac5.context, AutocompleteContext::Keyword);

    fixture.base.check(&String::from(
        r#"
        for x in y do  @1
    "#,
    ));

    let ac6 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac6.entry_map.get("in").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(ac6.context, AutocompleteContext::Statement);

    fixture.base.check(&String::from(
        r#"
        for x in y do e@1
    "#,
    ));

    let ac7 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac7.entry_map.get("in").map_or(0, |_| 1));
    assert_eq!(1, ac7.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(1, ac7.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(1, ac7.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(ac7.context, AutocompleteContext::Statement);
}
