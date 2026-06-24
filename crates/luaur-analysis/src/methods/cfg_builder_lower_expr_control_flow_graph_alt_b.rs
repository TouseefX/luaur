//! Source: `Analysis/src/ControlFlowGraph.cpp:362-366` (hand-ported)
//! C++ `void CFGBuilder::lowerExpr(AstExprLocal* local)`.
use crate::records::cfg_builder::CfgBuilder;
use crate::records::symbol::Symbol;
use crate::type_aliases::def_id_control_flow_graph::DefId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_local::AstExprLocal;

impl CfgBuilder {
    pub fn lower_expr_ast_expr_local(&mut self, local: *mut AstExprLocal) {
        unsafe {
            // C++:
            //   DefId def = readVariable(currentBlock, Symbol(local->local));
            //   cfg->useDefs[local] = def;
            let sym = Symbol::from_local((*local).local);
            let def: DefId = self.read_variable(self.current_block, sym);
            // useDefs is keyed by `AstExpr*`; `AstExprLocal*` upcasts to it.
            let key = local as *mut AstExpr;
            *self.cfg.as_mut().unwrap().use_defs.get_or_insert(key) = def;
        }
    }
}
