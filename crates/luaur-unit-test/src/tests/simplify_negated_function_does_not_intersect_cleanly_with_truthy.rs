//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:366:simplify_negated_function_does_not_intersect_cleanly_with_truthy`
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
//!   - calls -> method SimplifyFixture::isIntersection (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_negated_function_does_not_intersect_cleanly_with_truthy

#[cfg(test)]
#[test]
fn simplify_negated_function_does_not_intersect_cleanly_with_truthy() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let function_ty = fixture.function_ty;
    let truthy_ty = fixture.truthy_ty;

    let negated_function_ty = fixture.mk_negation(function_ty);
    let actual = fixture.intersect(negated_function_ty, truthy_ty);
    assert!(fixture.is_intersection(actual));
}
