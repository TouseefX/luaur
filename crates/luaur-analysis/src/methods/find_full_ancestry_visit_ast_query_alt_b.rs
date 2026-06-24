use crate::records::find_full_ancestry::FindFullAncestry;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_ast::visit::ast_expr_visit;

impl FindFullAncestry {
    pub fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
        let node_ref = unsafe { &*node };

        self.visit_ast_node(node as *mut AstNode);

        if unsafe { (*node_ref.name).base.location.contains(self.pos) } {
            unsafe {
                ast_expr_visit(node_ref.name, self);
            }
        } else if unsafe { (*node_ref.func).base.base.location.contains(self.pos) } {
            unsafe {
                ast_expr_visit(
                    node_ref.func as *mut luaur_ast::records::ast_expr::AstExpr,
                    self,
                );
            }
        }

        false
    }
}
