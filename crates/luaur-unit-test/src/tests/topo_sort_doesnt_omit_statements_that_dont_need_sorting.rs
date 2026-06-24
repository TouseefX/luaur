//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:61:topo_sort_doesnt_omit_statements_that_dont_need_sorting`
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
//!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item topo_sort_doesnt_omit_statements_that_dont_need_sorting

#[cfg(test)]
#[test]
fn topo_sort_doesnt_omit_statements_that_dont_need_sorting() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        local X = {}

        function A()
            return B(5), B("Hi")
        end

        local Y = {}

        function B(x)
            return x
        end

        local Z = B()
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    assert_eq!(5, sorted.len());

    let block = unsafe { &*program };
    assert_eq!(5, block.body.size);

    let x = unsafe { *block.body.data.add(0) };
    let a = unsafe { *block.body.data.add(1) };
    let y = unsafe { *block.body.data.add(2) };
    let b = unsafe { *block.body.data.add(3) };
    let z = unsafe { *block.body.data.add(4) };

    assert_eq!(sorted[0], x);
    assert_eq!(sorted[1], y);
    assert_eq!(sorted[2], b);
    assert_eq!(sorted[3], z);
    assert_eq!(sorted[4], a);
}
