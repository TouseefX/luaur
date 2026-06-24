//! Ported from upstream Luau doctest.
//! Node: `cxx:Test:Luau.UnitTest:tests/Subtyping.test.cpp:1887:subtyping_table_test_is_suppressing_if_all_mismatches_are_suppressing`
//! Source: `tests/Subtyping.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Subtyping.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/Instantiation2.h
//!   - includes -> source_file Analysis/include/Luau/TypeFwd.h
//!   - includes -> source_file Analysis/include/Luau/TypePath.h
//!   - includes -> source_file Analysis/include/Luau/Normalize.h
//!   - includes -> source_file Analysis/include/Luau/Subtyping.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//!   - includes -> source_file Analysis/include/Luau/TypePack.h
//!   - includes -> source_file Analysis/include/Luau/TypeFunction.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/RegisterCallbacks.h
//! - incoming:
//!   - declares <- source_file tests/Subtyping.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias TypeId (Analysis/include/Luau/TypeFwd.h)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record SubtypingResult (Analysis/include/Luau/Subtyping.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item subtyping_table_test_is_suppressing_if_all_mismatches_are_suppressing

#[cfg(test)]
#[test]
fn subtyping_table_test_is_suppressing_if_all_mismatches_are_suppressing() {
    use crate::records::subtype_fixture::SubtypeFixture;
    use alloc::sync::Arc;
    use luaur_analysis::records::scope::Scope;

    let mut fixture = SubtypeFixture::default();
    let table_one = fixture.base.parse_type("{foo: any, bar: any}");
    let table_two = fixture.base.parse_type("{foo: number, bar: string}");
    let root_scope = Arc::as_ptr(&fixture.root_scope) as *mut Scope;

    let sr = fixture
        .subtyping
        .is_subtype_type_id_type_id_not_null_scope(table_one, table_two, root_scope);

    assert!(!sr.is_subtype());
    assert!(sr.is_error_suppressing());
}
