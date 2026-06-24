//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Autocomplete.test.cpp:1370:autocomplete_type_scoping_easy`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record TableType (Analysis/include/Luau/Type.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item autocomplete_type_scoping_easy

#[cfg(test)]
#[test]
fn autocomplete_type_scoping_easy() {
    use crate::records::ac_fixture::AcFixture;
    use luaur_analysis::functions::follow_type::follow_type_id;
    use luaur_analysis::functions::get_type_alt_j::get_type_id;
    use luaur_analysis::records::table_type::TableType;

    let mut fixture = AcFixture::default();
    fixture.base.check(&String::from(
        r#"
type Table = { a: number, b: number }
do
    type Table = { x: string, y: string }
    local a: T@1
end
    "#,
    ));

    let ac = fixture.base.autocomplete_marker(b'1' as core::ffi::c_char);

    assert!(ac.entry_map.contains_key("Table"));
    let ty = ac.entry_map["Table"]
        .r#type
        .expect("Table entry should have a type");
    let ty = unsafe { follow_type_id(ty) };
    let table = unsafe { get_type_id::<TableType>(ty) };
    assert!(!table.is_null());
    assert!(unsafe { (*table).props.contains_key("x") });
}
