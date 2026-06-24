//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:610:normalize_intersect_truthy`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_intersect_truthy

#[cfg(test)]
#[test]
fn normalize_intersect_truthy() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = NormalizeFixture::default();
    assert_eq!(
        "number | string | true",
        to_string_type_id(fixture.normal(
            r#"
        (string | number | boolean | nil) & Not<false | nil>
    "#
        ))
    );
}
