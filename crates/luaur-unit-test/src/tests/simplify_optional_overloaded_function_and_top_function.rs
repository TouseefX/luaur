//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:349:simplify_optional_overloaded_function_and_top_function`
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
//!   - calls -> method SimplifyFixture::mkFunction (tests/Simplify.test.cpp)
//!   - type_ref -> record IntersectionType (Analysis/include/Luau/Type.h)
//!   - type_ref -> record UnionType (Analysis/include/Luau/Type.h)
//!   - calls -> method SimplifyFixture::mkNegation (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_optional_overloaded_function_and_top_function

#[cfg(test)]
#[test]
fn simplify_optional_overloaded_function_and_top_function() {
    use crate::records::simplify_fixture::SimplifyFixture;
    use alloc::vec;
    use luaur_analysis::records::intersection_type::IntersectionType;
    use luaur_analysis::records::union_type::UnionType;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;
    let nil_ty = fixture.nil_ty;
    let function_ty = fixture.function_ty;

    let f1 = fixture.mk_function(number_ty, string_ty);
    let f2 = fixture.mk_function(string_ty, number_ty);

    let f12 = fixture.arena.add_type(IntersectionType {
        parts: vec![f1, f2],
    });
    let t = fixture.arena.add_type(UnionType {
        options: vec![f12, nil_ty],
    });

    let not_function_ty = fixture.mk_negation(function_ty);

    let actual = fixture.intersect(t, not_function_ty);
    assert_eq!(nil_ty, actual);
    let actual = fixture.intersect(not_function_ty, t);
    assert_eq!(nil_ty, actual);
}
