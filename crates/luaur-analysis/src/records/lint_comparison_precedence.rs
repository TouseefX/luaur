use luaur_ast::records::ast_expr_binary::AstExprBinary;
use luaur_ast::records::ast_visitor::AstVisitor;

#[derive(Debug, Clone)]
pub struct LintComparisonPrecedence {
    pub(crate) context: *mut crate::records::lint_context::LintContext,
}

impl LintComparisonPrecedence {
    pub fn visit_node(&mut self, _node: *mut core::ffi::c_void) -> bool {
        true
    }
}

impl AstVisitor for LintComparisonPrecedence {
    fn visit_expr_binary(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_binary(node as *mut AstExprBinary)
    }

    fn visit_node(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_node(node)
    }

    fn visit_attr(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }

    fn visit_type(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }

    fn visit_type_pack(&mut self, _node: *mut core::ffi::c_void) -> bool {
        false
    }
}
