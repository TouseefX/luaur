//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2799:autocomplete_autocomplete_ifelse_expressions`
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
//!   - translates_to -> rust_item autocomplete_autocomplete_ifelse_expressions

#[cfg(test)]
#[test]
fn autocomplete_autocomplete_ifelse_expressions() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::enums::autocomplete_context::AutocompleteContext;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local temp = false
local even = true;
local a = true
a = if t@1emp then t
a = if temp t@2
a = if temp then e@3
a = if temp then even e@4
a = if temp then even elseif t@5
a = if temp then even elseif true t@6
a = if temp then even elseif true then t@7
a = if temp then even elseif true then temp e@8
a = if temp then even elseif true then temp else e@9
        "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac1.entry_map.contains_key("temp"));
    assert!(ac1.entry_map.contains_key("true"));
    assert!(!ac1.entry_map.contains_key("then"));
    assert!(!ac1.entry_map.contains_key("else"));
    assert!(!ac1.entry_map.contains_key("elseif"));
    assert_eq!(ac1.context, AutocompleteContext::Expression);

    let ac2 = fixture.base.autocomplete_marker(b'2' as core::ffi::c_char);
    assert!(!ac2.entry_map.contains_key("temp"));
    assert!(!ac2.entry_map.contains_key("true"));
    assert!(ac2.entry_map.contains_key("then"));
    assert!(!ac2.entry_map.contains_key("else"));
    assert!(!ac2.entry_map.contains_key("elseif"));
    assert_eq!(ac2.context, AutocompleteContext::Keyword);

    let ac3 = fixture.base.autocomplete_marker(b'3' as core::ffi::c_char);
    assert!(ac3.entry_map.contains_key("even"));
    assert!(!ac3.entry_map.contains_key("then"));
    assert!(!ac3.entry_map.contains_key("else"));
    assert!(!ac3.entry_map.contains_key("elseif"));
    assert_eq!(ac3.context, AutocompleteContext::Expression);

    let ac4 = fixture.base.autocomplete_marker(b'4' as core::ffi::c_char);
    assert!(!ac4.entry_map.contains_key("even"));
    assert!(!ac4.entry_map.contains_key("then"));
    assert!(ac4.entry_map.contains_key("else"));
    assert!(ac4.entry_map.contains_key("elseif"));
    assert_eq!(ac4.context, AutocompleteContext::Keyword);

    let ac5 = fixture.base.autocomplete_marker(b'5' as core::ffi::c_char);
    assert!(ac5.entry_map.contains_key("temp"));
    assert!(ac5.entry_map.contains_key("true"));
    assert!(!ac5.entry_map.contains_key("then"));
    assert!(!ac5.entry_map.contains_key("else"));
    assert!(!ac5.entry_map.contains_key("elseif"));
    assert_eq!(ac5.context, AutocompleteContext::Expression);

    let ac6 = fixture.base.autocomplete_marker(b'6' as core::ffi::c_char);
    assert!(!ac6.entry_map.contains_key("temp"));
    assert!(!ac6.entry_map.contains_key("true"));
    assert!(ac6.entry_map.contains_key("then"));
    assert!(!ac6.entry_map.contains_key("else"));
    assert!(!ac6.entry_map.contains_key("elseif"));
    assert_eq!(ac6.context, AutocompleteContext::Keyword);

    let ac7 = fixture.base.autocomplete_marker(b'7' as core::ffi::c_char);
    assert!(ac7.entry_map.contains_key("temp"));
    assert!(ac7.entry_map.contains_key("true"));
    assert!(!ac7.entry_map.contains_key("then"));
    assert!(!ac7.entry_map.contains_key("else"));
    assert!(!ac7.entry_map.contains_key("elseif"));
    assert_eq!(ac7.context, AutocompleteContext::Expression);

    let ac8 = fixture.base.autocomplete_marker(b'8' as core::ffi::c_char);
    assert!(!ac8.entry_map.contains_key("even"));
    assert!(!ac8.entry_map.contains_key("then"));
    assert!(ac8.entry_map.contains_key("else"));
    assert!(ac8.entry_map.contains_key("elseif"));
    assert_eq!(ac8.context, AutocompleteContext::Keyword);

    let ac9 = fixture.base.autocomplete_marker(b'9' as core::ffi::c_char);
    assert!(!ac9.entry_map.contains_key("then"));
    assert!(!ac9.entry_map.contains_key("else"));
    assert!(!ac9.entry_map.contains_key("elseif"));
    assert_eq!(ac9.context, AutocompleteContext::Expression);
}
