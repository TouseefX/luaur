//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:858:autocomplete_autocomplete_if_middle_keywords`
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
//!   - translates_to -> rust_item autocomplete_autocomplete_if_middle_keywords

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_if_middle_keywords() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        if   @1
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac1.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac1.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(1, ac1.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac1.context, AutocompleteContext::Expression);

    fixture.base.check(&String::from(
        r#"
        if x  @1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac2.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Keyword);

    fixture.base.check(&String::from(
        r#"
        if x t@1
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(3, ac3.entry_map.len());
    assert_eq!(1, ac3.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac3.entry_map.get("and").map_or(0, |_| 1));
    assert_eq!(1, ac3.entry_map.get("or").map_or(0, |_| 1));
    assert_eq!(ac3.context, AutocompleteContext::Keyword);

    fixture.base.check(&String::from(
        r#"
        if x then
@1
        end
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac4.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac4.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(1, ac4.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(1, ac4.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac4.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac4.context, AutocompleteContext::Statement);

    fixture.base.check(&String::from(
        r#"
        if x then
            t@1
        end
    "#,
    ));

    let ac4a = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac4a.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac4a.entry_map.get("table").map_or(0, |_| 1));
    assert_eq!(1, ac4a.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(1, ac4a.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(ac4a.context, AutocompleteContext::Statement);

    fixture.base.check(&String::from(
        r#"
        if x then
@1
        elseif x then
        end
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac5.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac5.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac5.context, AutocompleteContext::Statement);

    fixture.base.check(&String::from(
        r#"
        if t@1
    "#,
    ));

    let ac6 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac6.entry_map.get("true").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("false").map_or(0, |_| 1));
    assert_eq!(0, ac6.entry_map.get("then").map_or(0, |_| 1));
    assert_eq!(1, ac6.entry_map.get("function").map_or(0, |_| 1));
    assert_eq!(0, ac6.entry_map.get("else").map_or(0, |_| 1));
    assert_eq!(0, ac6.entry_map.get("elseif").map_or(0, |_| 1));
    assert_eq!(0, ac6.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac6.context, AutocompleteContext::Expression);
}
