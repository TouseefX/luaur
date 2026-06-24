use crate::records::expr_printer::ExprPrinter;
use luaur_ast::enums::constant_number_parse_result::ConstantNumberParseResult;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;

impl ExprPrinter {
    pub fn visit_ast_expr_constant_number(&mut self, node: *mut AstExprConstantNumber) -> bool {
        unsafe {
            let n = &*node;
            if n.parse_result == ConstantNumberParseResult::Ok && n.value == n.value.trunc() {
                self.result.push_str(&format!("{}", n.value as i64));
            } else {
                self.result.push_str(&format!("{}", n.value));
            }
        }
        false
    }
}
