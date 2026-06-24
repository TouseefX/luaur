//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:967:normalize_negations_of_tables`
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
//!   - calls -> method NormalizeFixture::toNormalizedType (tests/Normalize.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item normalize_negations_of_tables

#[cfg(test)]
#[test]
fn normalize_negations_of_tables() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = NormalizeFixture::default();
    let expected_errors = if !FFlag::DebugLuauForceOldSolver.get() {
        1
    } else {
        0
    };
    assert!(fixture
        .to_normalized_type("Not<{}>", expected_errors)
        .is_none());

    let expected = if FFlag::LuauIntegerType2.get() {
        "(boolean | buffer | function | integer | number | string | thread | userdata)?"
    } else {
        "(boolean | buffer | function | number | string | thread | userdata)?"
    };

    assert_eq!(expected, to_string_type_id(fixture.normal("Not<tbl>")));
    assert_eq!("table", to_string_type_id(fixture.normal("Not<Not<tbl>>")));
}
