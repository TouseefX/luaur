//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:960:normalize_top_table_type`
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
//!   - calls -> method NormalizeFixture::normal (tests/Normalize.test.cpp)
//!   - calls -> method SubtypeFixture::tbl (tests/Subtyping.test.cpp)
//!   - translates_to -> rust_item normalize_top_table_type

#[cfg(test)]
#[test]
fn normalize_top_table_type() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = NormalizeFixture::default();

    assert_eq!("table", to_string_type_id(fixture.normal("{} | tbl")));
    assert_eq!("{  }", to_string_type_id(fixture.normal("{} & tbl")));
    assert_eq!("never", to_string_type_id(fixture.normal("number & tbl")));
}
