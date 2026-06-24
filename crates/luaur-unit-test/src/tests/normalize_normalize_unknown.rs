//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1003:normalize_normalize_unknown`
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
//!   - calls -> method NormalizedType::isUnknown (Analysis/src/Normalize.cpp)
//!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_normalize_unknown

#[cfg(test)]
#[test]
fn normalize_normalize_unknown() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = NormalizeFixture::default();
    let nt = fixture
        .to_normalized_type("Not<string> | Not<number>", 0)
        .expect("expected normalized type");

    assert!(nt.is_unknown());
    assert_eq!(
        "unknown",
        to_string_type_id(fixture.type_from_normal(nt.as_ref()))
    );
}
