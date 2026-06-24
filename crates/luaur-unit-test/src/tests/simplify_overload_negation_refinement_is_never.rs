//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:122:simplify_overload_negation_refinement_is_never`
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
//!   - calls -> method SimplifyFixture::mkFunction (tests/Simplify.test.cpp)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_overload_negation_refinement_is_never

#[cfg(test)]
#[test]
fn simplify_overload_negation_refinement_is_never() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::vec;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let error_ty = fixture.error_ty;
    let function_ty = fixture.function_ty;
    let never_ty = fixture.never_ty;

    let f1 = fixture.mk_function(string_ty, number_ty);
    let f2 = fixture.mk_function(number_ty, string_ty);
    let intersection = fixture.arena.add_type(IntersectionType {
        parts: vec![f1, f2],
    });
    let union_t = fixture.arena.add_type(UnionType {
        options: vec![error_ty, function_ty],
    });
    let negation_t = fixture.mk_negation(union_t);

    let actual = fixture.intersect(intersection, negation_t);
    assert_eq!(never_ty, actual);
}
