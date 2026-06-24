//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:466:data_flow_graph_function_captures_are_phi_nodes_of_all_versions_properties`
//! Source: `tests/DataFlowGraph.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/DataFlowGraph.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/DataFlowGraph.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file Analysis/include/Luau/Def.h
//!   - includes -> source_file Analysis/include/Luau/Error.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/AstQueryDsl.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/DataFlowGraph.test.cpp
//! - outgoing:
//!   - calls -> method DataFlowGraphFixture::dfg (tests/DataFlowGraph.test.cpp)
//!   - calls -> function print (Analysis/src/TypeFunctionRuntime.cpp)
//!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
//!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
//!   - calls -> method RefinementKeyArena::node (Analysis/src/DataFlowGraph.cpp)
//!   - calls -> function query (tests/AstQueryDsl.h)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item data_flow_graph_function_captures_are_phi_nodes_of_all_versions_properties

#[cfg(test)]
#[test]
fn data_flow_graph_function_captures_are_phi_nodes_of_all_versions_properties() {
    use crate::functions::nth::nth_T;
    use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
    use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
    use luaur_ast::records::ast_expr_local::AstExprLocal;

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
        r#"
        local t = {}
        t.x = 5

        function f()
            print(t.x)
            t.x = nil
        end

        f()
        t.x = "five"
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(2)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(3)]);
    let x4 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(4)]);

    assert_ne!(x1, x2);
    assert_ne!(x2, x3);
    assert_ne!(x3, x4);

    let t1 = fixture.get_local_def(1, 0);
    let t2 = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(2)]);

    let phi = fixture.get_phi(t2);
    assert!(!phi.is_null());
    unsafe {
        assert_eq!((*phi).operands.len(), 1);
        assert_eq!((&(*phi).operands)[0], t1);
    }
}
