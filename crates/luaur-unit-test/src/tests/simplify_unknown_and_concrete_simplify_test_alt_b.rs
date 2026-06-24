//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:275:simplify_unknown_and_concrete`
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
//!   - translates_to -> rust_item simplify_unknown_and_concrete

#[cfg(test)]
#[test]
fn simplify_unknown_and_concrete() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let error_ty = fixture.error_ty;
    let true_ty = fixture.true_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(number_ty, error_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(error_ty, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(true_ty, error_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(error_ty, true_ty);
    assert_eq!(never_ty, actual);
}
