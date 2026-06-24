//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:646:autocomplete_autocomplete_for_middle_keywords`
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
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - translates_to -> rust_item autocomplete_autocomplete_for_middle_keywords

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_for_middle_keywords() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
        for x @1=
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac1.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac1.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac1.context, AutocompleteContext::Unknown);

    fixture.base.check(&String::from(
        r#"
        for x =@1 1
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac2.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac2.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac2.context, AutocompleteContext::Unknown);

    fixture.base.check(&String::from(
        r#"
        for x = 1,@1 2
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac3.entry_map.len());
    assert_eq!(1, ac3.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac3.context, AutocompleteContext::Keyword);

    fixture.base.check(&String::from(
        r#"
        for x = 1, @12,
    "#,
    ));

    let ac4 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(0, ac4.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac4.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac4.context, AutocompleteContext::Expression);

    fixture.base.check(&String::from(
        r#"
        for x = 1, 2, @15
    "#,
    ));

    let ac5 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac5.entry_map.get("math").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(0, ac5.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac5.context, AutocompleteContext::Expression);

    fixture.base.check(&String::from(
        r#"
        for x = 1, 2, 5 f@1
    "#,
    ));

    let ac6 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac6.entry_map.len());
    assert_eq!(1, ac6.entry_map.get("do").map_or(0, |_| 1));
    assert_eq!(ac6.context, AutocompleteContext::Keyword);

    fixture.base.check(&String::from(
        r#"
        for x = 1, 2, 5 do      @1
    "#,
    ));

    let ac7 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert_eq!(1, ac7.entry_map.get("end").map_or(0, |_| 1));
    assert_eq!(ac7.context, AutocompleteContext::Statement);

    fixture.base.check(&String::from(
        r#"local Foo = 1
        for x = @11, @22, @35
    "#,
    ));

    for i in 0..3 {
        let marker = (b'1' + i) as core::ffi::c_char;
        let ac8 = fixture.base.autocomplete_marker(marker);
        assert_eq!(1, ac8.entry_map.get("Foo").map_or(0, |_| 1));
        assert_eq!(0, ac8.entry_map.get("do").map_or(0, |_| 1));
    }

    fixture.base.check(&String::from(
        r#"local Foo = 1
        for x = @11, @22
    "#,
    ));

    for i in 0..2 {
        let marker = (b'1' + i) as core::ffi::c_char;
        let ac9 = fixture.base.autocomplete_marker(marker);
        assert_eq!(1, ac9.entry_map.get("Foo").map_or(0, |_| 1));
        assert_eq!(0, ac9.entry_map.get("do").map_or(0, |_| 1));
    }
}
