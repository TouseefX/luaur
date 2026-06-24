//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:421:simplify_tables_and_top_table`
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
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_tables_and_top_table

#[cfg(test)]
#[test]
fn simplify_tables_and_top_table() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let table_ty = fixture.table_ty;
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let never_ty = fixture.never_ty;

    let not_table_type = fixture.mk_negation(table_ty);
    let t1 = fixture.mk_table(&[("prop", string_ty), ("another", number_ty)]);

    let actual = fixture.intersect(t1, table_ty);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(table_ty, t1);
    assert_eq!(t1, actual);

    let actual = fixture.intersect(t1, not_table_type);
    assert_eq!(never_ty, actual);
    let actual = fixture.intersect(not_table_type, t1);
    assert_eq!(never_ty, actual);
}
