//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Frontend.test.cpp:1027:frontend_ast_node_at_position`
//! Source: `tests/Frontend.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Frontend.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file Analysis/include/Luau/BuiltinDefinitions.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Common/include/Luau/DenseHash.h
//!   - includes -> source_file Analysis/include/Luau/Frontend.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Analysis/include/Luau/RequireTracer.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Type.h
//! - incoming:
//!   - declares <- source_file tests/Frontend.test.cpp
//! - outgoing:
//!   - type_ref -> record SourceModule (Analysis/include/Luau/Module.h)
//!   - calls -> method Fixture::getMainSourceModule (tests/Fixture.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - type_ref -> record AstNode (Ast/include/Luau/Ast.h)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - translates_to -> rust_item frontend_ast_node_at_position

#[cfg(test)]
#[test]
fn frontend_ast_node_at_position() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::records::frontend_fixture::FrontendFixture;
    use alloc::string::String;
    use luaur_analysis::functions::find_node_at_position_ast_query::find_node_at_position_source_module_position;

    let mut fixture = FrontendFixture {
        base: BuiltinsFixture::default(),
    };

    fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {}

        function t:aa() end

        t:
    "#,
        ),
        None,
    );

    let module = fixture.base.base.get_main_source_module();
    let mut pos = unsafe { (*(*module).root).base.base.location.end };
    let node = unsafe { find_node_at_position_source_module_position(&*module, pos) };

    assert!(!node.is_null());
    assert!(unsafe { !(*node).as_expr().is_null() });

    pos.column += 1;
    let node2 = unsafe { find_node_at_position_source_module_position(&*module, pos) };
    assert_eq!(node, node2);
}
