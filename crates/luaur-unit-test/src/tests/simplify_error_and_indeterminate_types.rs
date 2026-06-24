//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:260:simplify_error_and_indeterminate_types`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::isIntersection (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_error_and_indeterminate_types

#[cfg(test)]
#[test]
fn simplify_error_and_indeterminate_types() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let error_ty = fixture.error_ty;
    let free_ty = fixture.free_ty;
    let generic_ty = fixture.generic_ty;
    let blocked_ty = fixture.blocked_ty;
    let pending_ty = fixture.pending_ty;

    let actual = fixture.intersect_str(error_ty, free_ty);
    assert_eq!("'a & *error-type*", actual);
    let actual = fixture.intersect_str(free_ty, error_ty);
    assert_eq!("'a & *error-type*", actual);

    let actual = fixture.intersect_str(error_ty, generic_ty);
    assert_eq!("*error-type* & b", actual);
    let actual = fixture.intersect_str(generic_ty, error_ty);
    assert_eq!("*error-type* & b", actual);

    let actual = fixture.intersect(error_ty, blocked_ty);
    assert!(fixture.is_intersection(actual));
    let actual = fixture.intersect(blocked_ty, error_ty);
    assert!(fixture.is_intersection(actual));

    let actual = fixture.intersect(error_ty, pending_ty);
    assert!(fixture.is_intersection(actual));
    let actual = fixture.intersect(pending_ty, error_ty);
    assert!(fixture.is_intersection(actual));
}
