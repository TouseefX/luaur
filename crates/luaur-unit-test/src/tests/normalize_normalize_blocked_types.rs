//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:977:normalize_normalize_blocked_types`
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
//!   - type_ref -> record BlockedType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - calls -> method NormalizeFixture::typeFromNormal (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_normalize_blocked_types

#[cfg(test)]
#[test]
fn normalize_normalize_blocked_types() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_analysis::records::blocked_type::BlockedType;

    let mut fixture = NormalizeFixture::default();
    let blocked = fixture.arena.add_type(BlockedType::default());

    let norm = fixture
        .normalize(blocked)
        .expect("expected normalized type");

    assert_eq!(blocked, fixture.type_from_normal(norm.as_ref()));
}
