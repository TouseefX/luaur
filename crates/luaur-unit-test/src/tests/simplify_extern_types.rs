//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:516:simplify_extern_types`
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
//!   - translates_to -> rust_item simplify_extern_types

#[cfg(test)]
#[test]
fn simplify_extern_types() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let child_class_ty = fixture.child_class_ty;
    let parent_class_ty = fixture.parent_class_ty;
    let unrelated_class_ty = fixture.unrelated_class_ty;
    let never_ty = fixture.never_ty;

    let actual = fixture.intersect(child_class_ty, parent_class_ty);
    assert_eq!(child_class_ty, actual);

    let actual = fixture.intersect(parent_class_ty, child_class_ty);
    assert_eq!(child_class_ty, actual);

    let actual = fixture.union_(child_class_ty, parent_class_ty);
    assert_eq!(parent_class_ty, actual);

    let actual = fixture.union_(parent_class_ty, child_class_ty);
    assert_eq!(parent_class_ty, actual);

    let actual = fixture.intersect(child_class_ty, unrelated_class_ty);
    assert_eq!(never_ty, actual);
}
