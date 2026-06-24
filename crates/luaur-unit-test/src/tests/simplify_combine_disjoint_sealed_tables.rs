//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:394:simplify_combine_disjoint_sealed_tables`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_combine_disjoint_sealed_tables

#[cfg(test)]
#[test]
fn simplify_combine_disjoint_sealed_tables() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;

    let t1 = fixture.mk_table(&[("prop", string_ty)]);
    let t2 = fixture.mk_table(&[("second_prop", number_ty)]);

    let actual = fixture.intersect_str(t1, t2);
    assert_eq!("{ prop: string, second_prop: number }", actual);
}
