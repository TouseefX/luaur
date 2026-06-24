//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:3769:autocomplete_string_contents_is_available_to_callback`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record GlobalTypes (Analysis/include/Luau/GlobalTypes.h)
//!   - calls -> method ACFixture::getFrontend (tests/Autocomplete.test.cpp)
//!   - calls -> function linearSearchForBinding (tests/Fixture.cpp)
//!   - calls -> method ACFixtureImpl::check (tests/Autocomplete.test.cpp)
//!   - type_ref -> record ExternType (Analysis/include/Luau/Type.h)
//!   - type_ref -> type_alias AutocompleteEntryMap (Analysis/include/Luau/AutocompleteTypes.h)
//!   - translates_to -> rust_item autocomplete_string_contents_is_available_to_callback

#[cfg(test)]
#[test]
fn autocomplete_string_contents_is_available_to_callback() {
    use crate::functions::autocomplete_attach_require_call_tag::autocomplete_attach_require_call_tag;
    use crate::records::ac_fixture::AcFixture;
    use alloc::rc::Rc;
    use alloc::string::String;
    use core::cell::Cell;

    let mut fixture = AcFixture::default();
    fixture.base.load_definition(&String::from(
        r#"
        declare function require(path: string): any
    "#,
    ));

    autocomplete_attach_require_call_tag(fixture.base.get_frontend());

    fixture.base.check(&String::from(
        r#"
        local x = require("testing/@1")
    "#,
    ));

    let is_correct = Rc::new(Cell::new(false));
    let is_correct_for_callback = Rc::clone(&is_correct);
    fixture.base.autocomplete_marker_callback(
        b'1' as core::ffi::c_char,
        Box::new(move |_tag, _extern_type, contents| {
            is_correct_for_callback.set(contents.as_deref() == Some("testing/"));
            None
        }),
    );

    assert!(is_correct.get());
}
