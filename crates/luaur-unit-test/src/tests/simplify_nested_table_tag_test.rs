//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:452:simplify_nested_table_tag_test`
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
//!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_nested_table_tag_test

#[cfg(test)]
#[test]
fn simplify_nested_table_tag_test() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let hello_ty = fixture.hello_ty;
    let number_ty = fixture.number_ty;
    let string_ty = fixture.string_ty;

    let subtable1 = fixture.mk_table(&[("tag", hello_ty), ("subprop", number_ty)]);
    let t1 = fixture.mk_table(&[("subtable", subtable1), ("prop", string_ty)]);
    let subtable2 = fixture.mk_table(&[("tag", hello_ty)]);
    let t2 = fixture.mk_table(&[("subtable", subtable2)]);

    let actual = fixture.intersect(t1, t2);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(t2, t1);
    assert_eq!(t1, actual);
}
