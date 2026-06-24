//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:912:normalize_negations_of_extern_types`
//! Source: `tests/Normalize.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Normalize.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file tests/ScopedFlags.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//! - incoming:
//!   - declares <- source_file tests/Normalize.test.cpp
//! - outgoing:
//!   - calls -> function createSomeExternTypes (tests/Fixture.cpp)
//!   - calls -> method NormalizeFixture::getFrontend (tests/Normalize.test.cpp)
//!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item normalize_negations_of_extern_types

#[cfg(test)]
#[test]
fn normalize_negations_of_extern_types() {
    use crate::functions::create_some_extern_types::create_some_extern_types;
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = NormalizeFixture::default();
    create_some_extern_types(fixture.get_frontend());

    assert_eq!(
        "(Parent & ~Child) | Unrelated",
        to_string_type_id(fixture.normal("(Parent & Not<Child>) | Unrelated"))
    );

    if FFlag::LuauIntegerType2.get() {
        assert_eq!(
            "((userdata & ~Child) | boolean | buffer | function | integer | number | string | table | thread)?",
            to_string_type_id(fixture.normal("Not<Child>"))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.normal("Not<Parent> & Child"))
        );
        assert_eq!(
            "((userdata & ~Parent) | Child | boolean | buffer | function | integer | number | string | table | thread)?",
            to_string_type_id(fixture.normal("Not<Parent> | Child"))
        );
        assert_eq!(
            "(boolean | buffer | function | integer | number | string | table | thread)?",
            to_string_type_id(fixture.normal("Not<cls>"))
        );
        assert_eq!(
            "(Parent | Unrelated | boolean | buffer | function | integer | number | string | table | thread)?",
            to_string_type_id(fixture.normal("Not<cls & Not<Parent> & Not<Child> & Not<Unrelated>>"))
        );
    } else {
        assert_eq!(
            "((userdata & ~Child) | boolean | buffer | function | number | string | table | thread)?",
            to_string_type_id(fixture.normal("Not<Child>"))
        );
        assert_eq!(
            "never",
            to_string_type_id(fixture.normal("Not<Parent> & Child"))
        );
        assert_eq!(
            "((userdata & ~Parent) | Child | boolean | buffer | function | number | string | table | thread)?",
            to_string_type_id(fixture.normal("Not<Parent> | Child"))
        );
        assert_eq!(
            "(boolean | buffer | function | number | string | table | thread)?",
            to_string_type_id(fixture.normal("Not<cls>"))
        );
        assert_eq!(
            "(Parent | Unrelated | boolean | buffer | function | number | string | table | thread)?",
            to_string_type_id(fixture.normal("Not<cls & Not<Parent> & Not<Child> & Not<Unrelated>>"))
        );
    }

    assert_eq!(
        "Child",
        to_string_type_id(fixture.normal("(Child | Unrelated) & Not<Unrelated>"))
    );
}
