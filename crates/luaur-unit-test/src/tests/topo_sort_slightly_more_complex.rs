//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:99:topo_sort_slightly_more_complex`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> function toposort (tests/TopoSort.test.cpp)
//!   - translates_to -> rust_item topo_sort_slightly_more_complex

#[cfg(test)]
#[test]
fn topo_sort_slightly_more_complex() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        local T = {}

        function T:foo()
            return T:bar(999), T:bar("hi")
        end

        function T:bar(i)
            return i
        end
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(3, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(2) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(1) });
}
