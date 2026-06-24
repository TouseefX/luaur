//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:190:topo_sort_dont_reorder_function_after_assignment_to_global`
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
//!   - calls -> function toposort (tests/TopoSort.test.cpp)
//!   - translates_to -> rust_item topo_sort_dont_reorder_function_after_assignment_to_global

#[cfg(test)]
#[test]
fn topo_sort_dont_reorder_function_after_assignment_to_global() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        local f

        function g()
            f()
        end

        f = function() end
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(3, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(1) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(2) });
}
