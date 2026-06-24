//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:133:simplify_unknown_and_other_tops_and_bottom_types`
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
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_unknown_and_other_tops_and_bottom_types

#[cfg(test)]
#[test]
fn simplify_unknown_and_other_tops_and_bottom_types() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let unknown_ty = fixture.unknown_ty;
    let any_ty = fixture.any_ty;
    let never_ty = fixture.never_ty;
    let error_ty = fixture.error_ty;

    let actual = fixture.intersect(unknown_ty, unknown_ty);
    assert_eq!(unknown_ty, actual);

    let actual = fixture.intersect_str(unknown_ty, any_ty);
    assert_eq!("any", actual);
    let actual = fixture.intersect_str(any_ty, unknown_ty);
    assert_eq!("any", actual);

    let actual = fixture.intersect(unknown_ty, never_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(never_ty, unknown_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(unknown_ty, error_ty);
    assert_eq!(error_ty, actual);
    let actual = fixture.intersect(error_ty, unknown_ty);
    assert_eq!(error_ty, actual);
}
