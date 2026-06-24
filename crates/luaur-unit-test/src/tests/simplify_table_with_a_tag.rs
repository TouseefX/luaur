//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:441:simplify_table_with_a_tag`
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
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> method SimplifyFixture::mkTable (tests/Simplify.test.cpp)
//!   - calls -> method SimplifyFixture::intersectStr (tests/Simplify.test.cpp)
//!   - translates_to -> rust_item simplify_table_with_a_tag

#[cfg(test)]
#[test]
fn simplify_table_with_a_tag() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let number_ty = fixture.number_ty;
    let hello_ty = fixture.hello_ty;

    let t1 = fixture.mk_table(&[("tag", string_ty), ("prop", number_ty)]);
    let t2 = fixture.mk_table(&[("tag", hello_ty)]);

    let actual = fixture.intersect_str(t1, t2);
    assert_eq!("{ prop: number, tag: string } & { tag: \"hello\" }", actual);
    let actual = fixture.intersect_str(t2, t1);
    assert_eq!("{ prop: number, tag: string } & { tag: \"hello\" }", actual);
}
