//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:527:simplify_negations_of_extern_types`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_negations_of_extern_types

#[cfg(test)]
#[test]
fn simplify_negations_of_extern_types() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let child_class_ty = fixture.child_class_ty;
    let parent_class_ty = fixture.parent_class_ty;
    let never_ty = fixture.never_ty;

    let not_child_class_ty = fixture.mk_negation(child_class_ty);
    let not_parent_class_ty = fixture.mk_negation(parent_class_ty);

    let actual = fixture.intersect(child_class_ty, not_parent_class_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(not_parent_class_ty, child_class_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect_str(not_child_class_ty, parent_class_ty);
    assert_eq!("Parent & ~Child", actual);

    let actual = fixture.intersect_str(parent_class_ty, not_child_class_ty);
    assert_eq!("Parent & ~Child", actual);

    let actual = fixture.intersect(not_child_class_ty, not_parent_class_ty);
    assert_eq!(not_parent_class_ty, actual);

    let actual = fixture.intersect(not_parent_class_ty, not_child_class_ty);
    assert_eq!(not_parent_class_ty, actual);
}
