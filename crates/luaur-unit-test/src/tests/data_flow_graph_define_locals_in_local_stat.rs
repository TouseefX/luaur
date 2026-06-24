//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:62:data_flow_graph_define_locals_in_local_stat`
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
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item data_flow_graph_define_locals_in_local_stat

#[cfg(test)]
#[test]
fn data_flow_graph_define_locals_in_local_stat() {
    use crate::functions::nth::nth_T;
    use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
    use luaur_ast::records::ast_expr_local::AstExprLocal;

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
        r#"
        local x = 5
        local y = x
    "#,
    );

    let _ = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(1)]);
}
