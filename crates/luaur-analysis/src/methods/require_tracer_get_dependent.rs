use crate::records::require_tracer::RequireTracer;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_expr_index_expr::AstExprIndexExpr;
use luaur_ast::records::ast_expr_index_name::AstExprIndexName;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_ast::records::ast_expr_type_assertion::AstExprTypeAssertion;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_type_group::AstTypeGroup;
use luaur_ast::records::ast_type_typeof::AstTypeTypeof;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl RequireTracer {
    pub fn get_dependent(&self, node: *mut AstNode) -> *mut AstNode {
        unsafe {
            if (*node).is::<AstExprLocal>() {
                let expr = node as *mut AstExprLocal;
                let local = (*expr).local;
                match self.locals.find(&local) {
                    Some(&val) => return val as *mut AstNode,
                    None => return core::ptr::null_mut(),
                }
            } else if (*node).is::<AstExprIndexName>() {
                let expr = node as *mut AstExprIndexName;
                return (*expr).expr as *mut AstNode;
            } else if (*node).is::<AstExprIndexExpr>() {
                let expr = node as *mut AstExprIndexExpr;
                return (*expr).expr as *mut AstNode;
            } else if (*node).is::<AstExprCall>() {
                let expr = node as *mut AstExprCall;
                if (*expr).self_ {
                    let func = (*expr).func as *mut AstExprIndexName;
                    return (*func).expr as *mut AstNode;
                }
            } else if (*node).is::<AstExprGroup>() {
                let expr = node as *mut AstExprGroup;
                return (*expr).expr as *mut AstNode;
            } else if (*node).is::<AstExprTypeAssertion>() {
                let expr = node as *mut AstExprTypeAssertion;
                return (*expr).annotation as *mut AstNode;
            } else if (*node).is::<AstTypeGroup>() {
                let expr = node as *mut AstTypeGroup;
                return (*expr).type_ as *mut AstNode;
            } else if (*node).is::<AstTypeTypeof>() {
                let expr = node as *mut AstTypeTypeof;
                return (*expr).expr as *mut AstNode;
            }
        }
        core::ptr::null_mut()
    }
}
