//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:333:simplify_negated_top_function_type`
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
//!   - calls -> method SimplifyFixture::mkFunction (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_negated_top_function_type

#[cfg(test)]
#[test]
fn simplify_negated_top_function_type() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let function_ty = fixture.function_ty;
    let number_ty = fixture.number_ty;
    let falsy_ty = fixture.falsy_ty;
    let string_ty = fixture.string_ty;
    let never_ty = fixture.never_ty;

    let negated_function_ty = fixture.mk_negation(function_ty);

    let actual = fixture.intersect(number_ty, negated_function_ty);
    assert_eq!(number_ty, actual);
    let actual = fixture.intersect(negated_function_ty, number_ty);
    assert_eq!(number_ty, actual);

    let actual = fixture.intersect(falsy_ty, negated_function_ty);
    assert_eq!(falsy_ty, actual);
    let actual = fixture.intersect(negated_function_ty, falsy_ty);
    assert_eq!(falsy_ty, actual);

    let f = fixture.mk_function(string_ty, number_ty);

    let actual = fixture.intersect(f, negated_function_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(negated_function_ty, f);
    assert_eq!(never_ty, actual);
}
