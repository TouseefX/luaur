//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:156:simplify_boolean_singletons`
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
//!   - calls -> method SimplifyFixture::union_ (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_boolean_singletons

#[cfg(test)]
#[test]
fn simplify_boolean_singletons() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let true_ty = fixture.true_ty;
    let false_ty = fixture.false_ty;
    let boolean_ty = fixture.boolean_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(true_ty, boolean_ty);
    assert_eq!(true_ty, actual);
    let actual = fixture.intersect(boolean_ty, true_ty);
    assert_eq!(true_ty, actual);

    let actual = fixture.intersect(false_ty, boolean_ty);
    assert_eq!(false_ty, actual);
    let actual = fixture.intersect(boolean_ty, false_ty);
    assert_eq!(false_ty, actual);

    let actual = fixture.intersect(false_ty, true_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(true_ty, false_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.union_(true_ty, boolean_ty);
    assert_eq!(boolean_ty, actual);
    let actual = fixture.union_(boolean_ty, true_ty);
    assert_eq!(boolean_ty, actual);
    let actual = fixture.union_(false_ty, boolean_ty);
    assert_eq!(boolean_ty, actual);
    let actual = fixture.union_(boolean_ty, false_ty);
    assert_eq!(boolean_ty, actual);
    let actual = fixture.union_(false_ty, true_ty);
    assert_eq!(boolean_ty, actual);
}
