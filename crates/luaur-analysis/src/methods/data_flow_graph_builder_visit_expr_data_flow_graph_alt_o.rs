use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::data_flow_result::DataFlowResult;
use luaur_ast::records::ast_expr_instantiate::AstExprInstantiate;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::FFlag;

impl DataFlowGraphBuilder {
    pub fn visit_expr_ast_expr_instantiate(
        &mut self,
        i: *mut AstExprInstantiate,
    ) -> DataFlowResult {
        unsafe {
            if FFlag::LuauExplicitTypeInstantiationSupport.get() {
                for type_or_pack in (*i).type_arguments.iter() {
                    if !type_or_pack.r#type.is_null() {
                        self.visit_type_ast_type(type_or_pack.r#type);
                    } else {
                        LUAU_ASSERT!(!type_or_pack.type_pack.is_null());
                        self.visit_type_pack_ast_type_pack(type_or_pack.type_pack);
                    }
                }
            }

            self.visit_expr_ast_expr((*i).expr)
        }
    }
}
