//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/TopoSort.test.cpp:416:topo_sort_continue_comes_last`
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
//!   - type_ref -> record AstStatRepeat (Ast/include/Luau/Ast.h)
//!   - calls -> function toposort (tests/TopoSort.test.cpp)
//!   - translates_to -> rust_item topo_sort_continue_comes_last

#[cfg(test)]
#[test]
fn topo_sort_continue_comes_last() {
    use crate::functions::toposort::toposort;
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_repeat::AstStatRepeat;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::rtti::ast_node_as;

    let mut fixture = Fixture::default();
    let program = fixture.parse(
        r#"
repeat
local module = {}
local function confuseCompiler() return module.foo() end
module.foo = function() return "" end
continue
until true
    "#,
        &ParseOptions::default(),
    );

    let program = unsafe { &*program };
    assert_eq!(program.body.size, 1);

    let repeat = unsafe { ast_node_as::<AstStatRepeat>(*program.body.data.add(0) as *mut AstNode) };
    assert!(!repeat.is_null());

    let body = unsafe { &mut *(*repeat).body };
    assert_eq!(body.body.size, 4);

    let sorted = toposort(body);

    assert_eq!(sorted.len(), 4);
    assert_eq!(sorted[3], unsafe { *body.body.data.add(3) });
}
