//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:284:data_flow_graph_mutate_preexisting_property_not_owned_by_while`
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
//!   - calls -> method IrBuilder::cond (CodeGen/src/IrBuilder.cpp)
//!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
//!   - type_ref -> record AstExprIndexName (Ast/include/Luau/Ast.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
//!   - calls -> method DataFlowGraphFixture::checkOperands (tests/DataFlowGraph.test.cpp)
//!   - translates_to -> rust_item data_flow_graph_mutate_preexisting_property_not_owned_by_while

#[cfg(test)]
#[test]
fn data_flow_graph_mutate_preexisting_property_not_owned_by_while() {
    use crate::functions::nth::nth_T;
    use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
    use luaur_ast::records::ast_expr_index_name::AstExprIndexName;

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
        r#"
        local t = {}
        t.x = 5

        while cond() do
            t.x = true
        end

        local y = t.x
    "#,
    );

    let x1 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(1)]);
    let x2 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(2)]);
    let x3 = fixture.get_def::<AstExprIndexName>(vec![nth_T::<AstExprIndexName>(3)]);

    let phi = fixture.get_phi(x3);
    assert!(!phi.is_null());
    fixture.check_operands(phi, vec![x1, x2]);
}
