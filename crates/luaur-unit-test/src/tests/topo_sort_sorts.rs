//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:21:topo_sort_sorts`
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
//!   - translates_to -> rust_item topo_sort_sorts

#[cfg(test)]
#[test]
fn topo_sort_sorts() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
        function A()
            return B("high five!")
        end

        function B(x)
            return x
        end
    "#,
        &ParseOptions::default(),
    );

    let sorted = unsafe { toposort(&mut *program) };
    assert_eq!(2, sorted.len());

    let block = unsafe { &*program };
    assert_eq!(2, block.body.size);

    assert_eq!(unsafe { *block.body.data.add(1) }, sorted[0]);
    assert_eq!(unsafe { *block.body.data.add(0) }, sorted[1]);
}
