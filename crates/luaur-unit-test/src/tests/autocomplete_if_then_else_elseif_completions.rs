//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:2528:autocomplete_if_then_else_elseif_completions`
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
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - translates_to -> rust_item autocomplete_if_then_else_elseif_completions

#[cfg(test)]
#[test]
fn autocomplete_if_then_else_elseif_completions() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
local elsewhere = false

if true then
    return 1
el@1
end
    "#,
    ));

    let ac1 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac1.entry_map.contains_key("else"));
    assert!(ac1.entry_map.contains_key("elseif"));
    assert!(!ac1.entry_map.contains_key("elsewhere"));

    fixture.base.check(&String::from(
        r#"
local elsewhere = false

if true then
    return 1
else
    return 2
el@1
end
    "#,
    ));

    let ac2 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(!ac2.entry_map.contains_key("else"));
    assert!(!ac2.entry_map.contains_key("elseif"));
    assert!(ac2.entry_map.contains_key("elsewhere"));

    fixture.base.check(&String::from(
        r#"
local elsewhere = false

if true then
    print("1")
elif true then
    print("2")
el@1
end
    "#,
    ));

    let ac3 = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);
    assert!(ac3.entry_map.contains_key("else"));
    assert!(ac3.entry_map.contains_key("elseif"));
    assert!(ac3.entry_map.contains_key("elsewhere"));
}
