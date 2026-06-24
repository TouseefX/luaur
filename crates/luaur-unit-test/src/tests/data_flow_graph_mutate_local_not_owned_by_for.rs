//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:206:data_flow_graph_mutate_local_not_owned_by_for`
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
//!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
//!   - calls -> function query (tests/AstQueryDsl.h)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
//!   - calls -> method DataFlowGraphFixture::checkOperands (tests/DataFlowGraph.test.cpp)
//!   - translates_to -> rust_item data_flow_graph_mutate_local_not_owned_by_for

#[cfg(test)]
#[test]
fn data_flow_graph_mutate_local_not_owned_by_for() {
    use crate::functions::nth::nth_T;
    use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
    use luaur_ast::records::ast_expr_local::AstExprLocal;

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
        r#"
        local x

        for i = 0, 5 do
            x = true
        end

        local y = x
    "#,
    );

    let x0 = fixture.get_local_def(1, 0);
    let x1 = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(1)]);
    let x2 = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(2)]);

    let phi = fixture.get_phi(x2);
    assert!(!phi.is_null());
    fixture.check_operands(phi, vec![x0, x1]);
}
