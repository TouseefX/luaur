//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:603:data_flow_graph_phi_node_if_case_table_prop`
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
//!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item data_flow_graph_phi_node_if_case_table_prop

#[cfg(test)]
#[test]
fn data_flow_graph_phi_node_if_case_table_prop() {
    use crate::functions::nth::nth_T;
    use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
    use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
        r#"
local t = {}
t.x = true
if true then
    if true then
        t.x = 5
    end
    print(t.x)
else
    print(t.x)
end
"#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(2)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(3)]);

    let phi = fixture.get_phi(x3);
    assert!(!phi.is_null());
    unsafe {
        assert_eq!((*phi).operands.len(), 2);
        assert_eq!((&(*phi).operands)[0], x1);
        assert_eq!((&(*phi).operands)[1], x2);
    }
}
