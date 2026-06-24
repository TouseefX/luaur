//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:500:simplify_negations`
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
//!   - translates_to -> rust_item simplify_negations

#[cfg(test)]
#[test]
fn simplify_negations() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;
    let never_ty = fixture.never_ty;

    let not_number_ty = fixture.mk_negation(number_ty);
    let not_string_ty = fixture.mk_negation(string_ty);

    let actual = fixture.intersect(number_ty, not_number_ty);
    assert_eq!(never_ty, actual);

    let actual = fixture.intersect(number_ty, not_string_ty);
    assert_eq!(number_ty, actual);
    let actual = fixture.intersect(not_string_ty, number_ty);
    assert_eq!(number_ty, actual);
}
