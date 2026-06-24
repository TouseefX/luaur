use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

impl AutocompleteNodeFinder {
    pub fn visit_ast_expr(&mut self, expr: *mut AstExpr) -> bool {
        let expr_ref = unsafe { &*expr };
        let location = expr_ref.base.location;
        if location.begin <= self.pos && self.pos <= location.end && location.begin != location.end
        {
            self.ancestry
                .push(expr as *mut AstExpr as *mut luaur_ast::records::ast_node::AstNode);
            true
        } else {
            false
        }
    }
}
