//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:542:simplify_intersection_of_intersection_of_a_free_type_can_result_in_removal_of_that_free_type`
//! Source: `tests/Simplify.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Simplify.test.cpp
//! - source_includes:
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Simplify.h
//! - incoming:
//!   - declares <- source_file tests/Simplify.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_intersection_of_intersection_of_a_free_type_can_result_in_removal_of_that_free_type

#[cfg(test)]
#[test]
fn simplify_intersection_of_intersection_of_a_free_type_can_result_in_removal_of_that_free_type() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::vec;
    use luaur_analysis::records::intersection_type::IntersectionType;

    let mut fixture = SimplifyFixture::default();
    let free_ty = fixture.free_ty;
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let never_ty = fixture.never_ty;

    let t1 = fixture.arena.add_type(IntersectionType {
        parts: vec![free_ty, string_ty],
    });

    let actual = fixture.intersect(t1, number_ty);
    assert_eq!(never_ty, actual);
}
