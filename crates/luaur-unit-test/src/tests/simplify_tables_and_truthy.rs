//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:433:simplify_tables_and_truthy`
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
//!   - translates_to -> rust_item simplify_tables_and_truthy

#[cfg(test)]
#[test]
fn simplify_tables_and_truthy() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let truthy_ty = fixture.truthy_ty;

    let t1 = fixture.mk_table(&[("prop", string_ty), ("another", number_ty)]);

    let actual = fixture.intersect(t1, truthy_ty);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(truthy_ty, t1);
    assert_eq!(t1, actual);
}
