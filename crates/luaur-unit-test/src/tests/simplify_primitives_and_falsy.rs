//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:307:simplify_primitives_and_falsy`
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
//!   - translates_to -> rust_item simplify_primitives_and_falsy

#[cfg(test)]
#[test]
fn simplify_primitives_and_falsy() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let falsy_ty = fixture.falsy_ty;
    let never_ty = fixture.never_ty;
    let nil_ty = fixture.nil_ty;

    let actual = fixture.intersect(number_ty, falsy_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(falsy_ty, number_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(nil_ty, falsy_ty);
    assert_eq!(nil_ty, actual);
    let actual = fixture.intersect(falsy_ty, nil_ty);
    assert_eq!(nil_ty, actual);
}
