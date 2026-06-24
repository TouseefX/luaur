//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:156:topo_sort_dont_reorder_assigns`
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
//!   - translates_to -> rust_item topo_sort_dont_reorder_assigns

#[cfg(test)]
#[test]
fn topo_sort_dont_reorder_assigns() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        local T = {}                -- 0

        function T.a()              -- 1 depends on (2)
            T.b()
        end

        function T.b()              -- 2 depends on (5)
            T.c()
        end

        function make_function()    -- 3
            return function() end
        end

        T.a()                       -- 4 depends on (1 -> 2 -> 5), but we cannot reorder it after 5!

        T.c = make_function()       -- 5 depends on (3)
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(6, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(3) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(2) });
    assert_eq!(sorted[3], unsafe { *program.body.data.add(1) });
    assert_eq!(sorted[4], unsafe { *program.body.data.add(4) });
    assert_eq!(sorted[5], unsafe { *program.body.data.add(5) });
}
