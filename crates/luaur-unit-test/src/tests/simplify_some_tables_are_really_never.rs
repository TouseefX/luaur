//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:552:simplify_some_tables_are_really_never`
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
//!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_some_tables_are_really_never

#[cfg(test)]
#[test]
fn simplify_some_tables_are_really_never() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let any_ty = fixture.any_ty;
    let unknown_ty = fixture.unknown_ty;
    let number_ty = fixture.number_ty;
    let never_ty = fixture.never_ty;

    let not_any_ty = fixture.mk_negation(any_ty);
    let t1 = fixture.mk_table(&[("someKey", not_any_ty)]);

    let actual = fixture.intersect(t1, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(number_ty, t1);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(t1, t1);
    assert_eq!(t1, actual);

    let not_unknown_ty = fixture.mk_negation(unknown_ty);
    let t2 = fixture.mk_table(&[("someKey", not_unknown_ty)]);

    let actual = fixture.intersect(t2, number_ty);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(number_ty, t2);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(t2, t2);
    assert_eq!(never_ty, actual);
}
