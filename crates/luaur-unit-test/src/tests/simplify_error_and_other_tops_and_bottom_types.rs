//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:249:simplify_error_and_other_tops_and_bottom_types`
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
//!   - translates_to -> rust_item simplify_error_and_other_tops_and_bottom_types

#[cfg(test)]
#[test]
fn simplify_error_and_other_tops_and_bottom_types() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let error_ty = fixture.error_ty;
    let any_ty = fixture.any_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(error_ty, error_ty);
    assert_eq!(error_ty, actual);

    let actual = fixture.intersect(error_ty, any_ty);
    assert_eq!(error_ty, actual);
    let actual = fixture.intersect(any_ty, error_ty);
    assert_eq!(error_ty, actual);

    let actual = fixture.intersect(error_ty, never_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(never_ty, error_ty);
    assert_eq!(never_ty, actual);
}
