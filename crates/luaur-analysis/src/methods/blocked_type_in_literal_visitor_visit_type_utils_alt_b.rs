use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_literal::is_literal;
use crate::records::blocked_type::BlockedType;
use crate::records::blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::rtti::ast_node_is;

impl BlockedTypeInLiteralVisitor {
    pub fn visit_ast_expr(&mut self, e: *mut AstExpr) -> bool {
        unsafe {
            if let Some(&ty) = (*self.ast_types).find(&(e as *const AstExpr)) {
                let followed = follow_type_id(ty);
                if !get_type_id::<BlockedType>(followed).is_null() {
                    (*self.to_block).push(ty);
                }
            }
        }
        is_literal(e as *const AstExpr) || unsafe { ast_node_is::<AstExprGroup>(&(*e).base) }
    }
}
