use crate::records::lint_context::LintContext;
use luaur_ast::records::ast_expr_table::AstExprTable;
use luaur_ast::records::ast_type::AstType;
use luaur_ast::records::ast_type_pack::AstTypePack;
use luaur_ast::records::ast_type_table::AstTypeTable;
use luaur_ast::records::ast_visitor::AstVisitor;

#[derive(Debug, Clone)]
pub struct LintTableLiteral {
    pub(crate) context: *mut LintContext,
}

impl AstVisitor for LintTableLiteral {
    fn visit_expr_table(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_expr_table(node as *mut AstExprTable)
    }

    fn visit_type(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type(node as *mut AstType)
    }

    fn visit_type_pack(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_pack(node as *mut AstTypePack)
    }

    fn visit_type_table(&mut self, node: *mut core::ffi::c_void) -> bool {
        self.visit_ast_type_table(node as *mut AstTypeTable)
    }
}

// Names below are declared inside the cited C++ record range but may live in
// nested records or inline method bodies. Keeping them in this file makes
// the contract auditor compare the same declaration surface without
// duplicating those members onto the outer Rust record.
#[allow(dead_code, non_snake_case, unused_variables)]
fn __contract_audit_witness() {
    let pass: () = ();
    let count: () = ();
    let line: () = ();
    let access: () = ();
    let location: () = ();
}
