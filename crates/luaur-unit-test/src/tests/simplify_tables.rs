//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Simplify.test.cpp:376:simplify_tables`
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
//!   - calls -> method SimplifyFixture::intersect (tests/Simplify.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - translates_to -> rust_item simplify_tables

#[cfg(test)]
#[test]
fn simplify_tables() {
    use crate::records::simplify_fixture::SimplifyFixture;

    let mut fixture = SimplifyFixture::default();
    let string_ty = fixture.string_ty;
    let table_ty = fixture.table_ty;
    let never_ty = fixture.never_ty;
    let function_ty = fixture.function_ty;
    let hello_ty = fixture.hello_ty;

    let t1 = fixture.mk_table(&[("tag", string_ty)]);

    let actual = fixture.intersect(t1, table_ty);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(t1, function_ty);
    assert_eq!(never_ty, actual);

    let t2 = fixture.mk_table(&[("tag", hello_ty)]);

    let actual = fixture.intersect(t1, t2);
    assert_eq!(t2, actual);
    let actual = fixture.intersect(t2, t1);
    assert_eq!(t2, actual);

    let t3 = fixture.mk_table(&[]);

    let actual = fixture.intersect(t1, t3);
    assert_eq!(t1, actual);
    let actual = fixture.intersect(t3, t1);
    assert_eq!(t1, actual);
}
