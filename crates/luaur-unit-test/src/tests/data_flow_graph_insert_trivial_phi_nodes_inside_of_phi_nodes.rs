//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/DataFlowGraph.test.cpp:650:data_flow_graph_insert_trivial_phi_nodes_inside_of_phi_nodes`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method DataFlowGraphFixture::getDef (tests/DataFlowGraph.test.cpp)
//!   - calls -> function query (tests/AstQueryDsl.h)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record Phi (Analysis/include/Luau/Def.h)
//!   - calls -> function get (tests/Fixture.h)
//!   - translates_to -> rust_item data_flow_graph_insert_trivial_phi_nodes_inside_of_phi_nodes

#[cfg(test)]
#[test]
fn data_flow_graph_insert_trivial_phi_nodes_inside_of_phi_nodes() {
    use crate::functions::nth::nth_T;
    use crate::records::data_flow_graph_fixture::DataFlowGraphFixture;
    use luaur_ast::records::ast_expr_local::AstExprLocal;

    let mut fixture = DataFlowGraphFixture::new();
    fixture.dfg(
        r#"
        local t = {}

        local function f(k: string)
            if t[k] ~= nil then
                return
            end

            t[k] = 5
        end
    "#,
    );

    let t1 = fixture.get_local_def(1, 0);
    let t2 = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(1)]);
    let t3 = fixture.get_def::<AstExprLocal>(vec![nth_T::<AstExprLocal>(3)]);

    assert_ne!(t1, t2);
    assert_eq!(t2, t3);

    let t2phi = fixture.get_phi(t2);
    assert!(!t2phi.is_null());
    unsafe {
        assert_eq!((*t2phi).operands.len(), 1);
        assert_eq!((&(*t2phi).operands)[0], t1);
    }
}
