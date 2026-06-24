//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3461:autocomplete_no_incompatible_self_calls_on_class`
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
//!   - calls -> method ACFixtureImpl::loadDefinition (tests/Autocomplete.test.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - translates_to -> rust_item autocomplete_no_incompatible_self_calls_on_class

#[cfg(test)]
#[test]
fn autocomplete_no_incompatible_self_calls_on_class() {
    use crate::records::ac_fixture::AcFixture;

    let mut fixture = AcFixture::default();
    fixture.base.load_definition(&String::from(
        r#"
declare class Foo
    function one(self): number
    two: () -> number
end
    "#,
    ));

    fixture.base.check(&String::from(
        r#"
local function f(t: Foo)
    t:@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("one"));
    assert!(ac.entry_map.contains_key("two"));
    assert!(!ac.entry_map["one"].wrong_index_type);
    assert!(ac.entry_map["two"].wrong_index_type);
    assert!(ac.entry_map["one"].indexed_with_self);
    assert!(ac.entry_map["two"].indexed_with_self);

    fixture.base.check(&String::from(
        r#"
local function f(t: Foo)
    t.@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("one"));
    assert!(ac.entry_map.contains_key("two"));
    assert!(ac.entry_map["one"].wrong_index_type);
    assert!(!ac.entry_map["two"].wrong_index_type);
    assert!(!ac.entry_map["one"].indexed_with_self);
    assert!(!ac.entry_map["two"].indexed_with_self);
}
