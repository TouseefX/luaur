//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:597:simplify_free_type_bound_by_any_with_any`
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
//!   - translates_to -> rust_item simplify_free_type_bound_by_any_with_any

#[cfg(test)]
#[test]
fn simplify_free_type_bound_by_any_with_any() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let free_ty = fixture.free_ty;
    let any_ty = fixture.any_ty;

    let actual = fixture.intersect_str(free_ty, any_ty);
    assert_eq!("'a | *error-type*", actual);
    let actual = fixture.intersect_str(any_ty, free_ty);
    assert_eq!("'a | *error-type*", actual);

    let actual = fixture.intersect_str(free_ty, any_ty);
    assert_eq!("'a | *error-type*", actual);
    let actual = fixture.intersect_str(any_ty, free_ty);
    assert_eq!("'a | *error-type*", actual);
}
