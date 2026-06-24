//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:226:simplify_unknown_and_indeterminate_types`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_unknown_and_indeterminate_types

#[cfg(test)]
#[test]
fn simplify_unknown_and_indeterminate_types() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let unknown_ty = fixture.unknown_ty;
    let free_ty = fixture.free_ty;
    let generic_ty = fixture.generic_ty;
    let blocked_ty = fixture.blocked_ty;
    let pending_ty = fixture.pending_ty;

    let actual = fixture.intersect(unknown_ty, free_ty);
    assert_eq!(free_ty, actual);
    let actual = fixture.intersect(free_ty, unknown_ty);
    assert_eq!(free_ty, actual);

    let actual = fixture.intersect(unknown_ty, generic_ty);
    assert_eq!(generic_ty, actual);
    let actual = fixture.intersect(generic_ty, unknown_ty);
    assert_eq!(generic_ty, actual);

    let actual = fixture.intersect(unknown_ty, blocked_ty);
    assert_eq!(blocked_ty, actual);
    let actual = fixture.intersect(unknown_ty, blocked_ty);
    assert_eq!(blocked_ty, actual);

    let actual = fixture.intersect(unknown_ty, pending_ty);
    assert_eq!(pending_ty, actual);
    let actual = fixture.intersect(unknown_ty, pending_ty);
    assert_eq!(pending_ty, actual);
}
