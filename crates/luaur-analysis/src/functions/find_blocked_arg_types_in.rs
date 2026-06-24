use crate::functions::is_literal::is_literal;
use crate::records::blocked_type_in_literal_visitor::BlockedTypeInLiteralVisitor;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_group::AstExprGroup;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_ast::rtti::ast_node_is;
use luaur_ast::visit::ast_expr_visit;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl AstVisitor for BlockedTypeInLiteralVisitor {
    fn visit_node(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_node(node as *mut AstNode)
    }

    fn visit_expr(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr(node as *mut AstExpr)
    }
}

pub fn find_blocked_arg_types_in(
    expr: *mut AstExprCall,
    ast_types: *mut DenseHashMap<*const AstExpr, TypeId>,
) -> Vec<TypeId> {
    let mut to_block: Vec<TypeId> = Vec::new();
    let mut v = BlockedTypeInLiteralVisitor {
        ast_types,
        to_block: &mut to_block as *mut Vec<TypeId>,
    };
    unsafe {
        for &arg in (*expr).args.iter() {
            if is_literal(arg as *const AstExpr) || ast_node_is::<AstExprGroup>(&(*arg).base) {
                ast_expr_visit(arg, &mut v);
            }
        }
    }
    to_block
}
