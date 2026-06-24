//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:525:data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_prior_versions`
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
//!   - calls -> function query (tests/AstQueryDsl.h)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_prior_versions

#[cfg(test)]
#[test]
fn data_flow_graph_local_f_which_is_prototyped_enclosed_by_function_has_some_prior_versions() {
    use crate::functions::nth::nth_T;
    use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
    use luaur_ast::records::ast_expr_local::AstExprLocal;

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
        r#"
        local f
        f = 5
        function f()
            if cond() then
                f()
            end
        end
    "#,
    );

    let f1 = fixture.get_local_def(1, 0);
    let f2 = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(1)]);
    let f3 = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(2)]);
    let f4 = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(3)]);

    assert_ne!(f1, f2);
    assert_ne!(f2, f3);
    assert_ne!(f3, f4);

    let phi = fixture.get_phi(f4);
    assert!(!phi.is_null());
    unsafe {
        assert_eq!((*phi).operands.len(), 1);
        assert_eq!((&(*phi).operands)[0], f3);
    }
}
