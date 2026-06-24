//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:211:topo_sort_local_functions_need_sorting_too`
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
//!   - translates_to -> rust_item topo_sort_local_functions_need_sorting_too

#[cfg(test)]
#[test]
fn topo_sort_local_functions_need_sorting_too() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        local a = nil                       -- 0

        local function f()                  -- 1 depends on 4
            a.c = 4
        end

        local function g()                  -- 2 depends on 1
            f()
        end

        a = {}                              -- 3
        a.c = nil                           -- 4
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    let program = unsafe { &*program };

    assert_eq!(5, sorted.len());
    assert_eq!(sorted[0], unsafe { *program.body.data.add(0) });
    assert_eq!(sorted[1], unsafe { *program.body.data.add(3) });
    assert_eq!(sorted[2], unsafe { *program.body.data.add(4) });
    assert_eq!(sorted[3], unsafe { *program.body.data.add(1) });
    assert_eq!(sorted[4], unsafe { *program.body.data.add(2) });
}
