//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:1035:normalize_final_types_are_cached`
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
//!   - type_ref -> record NormalizedType (Analysis/include/Luau/Normalize.h)
//!   - calls -> method NormalizeFixture::normalize (tests/Normalize.test.cpp)
//!   - translates_to -> rust_item normalize_final_types_are_cached

#[cfg(test)]
#[test]
fn normalize_final_types_are_cached() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use alloc::sync::Arc;

    let mut fixture = NormalizeFixture::default();
    let number_type = fixture.base.get_builtins().numberType;

    let na1 = fixture
        .normalize(number_type)
        .expect("expected normalized number");
    let na2 = fixture
        .normalize(number_type)
        .expect("expected normalized number");

    assert!(Arc::ptr_eq(&na1, &na2));
}
