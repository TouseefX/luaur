//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Normalize.test.cpp:724:normalize_specific_functions_cannot_be_negated`
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
//!   - translates_to -> rust_item normalize_specific_functions_cannot_be_negated

#[cfg(test)]
#[test]
fn normalize_specific_functions_cannot_be_negated() {
    use crate::records::normalize_fixture::NormalizeFixture;
    use luaur_common::FFlag;

    let mut fixture = NormalizeFixture::default();
    let expected_errors = if !FFlag::DebugLuauForceOldSolver.get() {
        1
    } else {
        0
    };

    assert!(fixture
        .to_normalized_type("Not<(boolean) -> boolean>", expected_errors)
        .is_none());
}
