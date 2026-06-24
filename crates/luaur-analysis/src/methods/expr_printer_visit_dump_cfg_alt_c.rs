use crate::records::expr_printer::ExprPrinter;
use luaur_ast::records::ast_expr_constant_string::AstExprConstantString;

impl ExprPrinter {
    pub fn visit_ast_expr_constant_string(&mut self, node: *mut AstExprConstantString) -> bool {
        unsafe {
            let n = &*node;
            self.result.push('"');
            let s = core::slice::from_raw_parts(n.value.data as *const u8, n.value.size as usize);
            self.result
                .push_str(&alloc::string::String::from_utf8_lossy(s));
            self.result.push('"');
        }
        false
    }
}
