//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:319:topo_sort_nested_type_annotations_depends_on_later_typealiases`
//! Source: `tests/TopoSort.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/TopoSort.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/TopoSortStatements.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/TopoSort.test.cpp
//! - outgoing:
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function toposort (tests/TopoSort.test.cpp)
//!   - translates_to -> rust_item topo_sort_nested_type_annotations_depends_on_later_typealiases

#[cfg(test)]
#[test]
fn topo_sort_nested_type_annotations_depends_on_later_typealiases() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        type Foo = A | B
        type B = number
        type A = string
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(3, sorted.len());

    let foo = unsafe { *program.body.data.add(0) };
    let b = unsafe { *program.body.data.add(1) };
    let a = unsafe { *program.body.data.add(2) };

    assert_eq!(sorted[0], b);
    assert_eq!(sorted[1], a);
    assert_eq!(sorted[2], foo);
}
