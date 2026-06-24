use crate::functions::emit_warning::emit_warning;
use crate::records::lint_multi_line_statement::LintMultiLineStatement;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_config::enums::code::Code;
use luaur_config::records::lint_warning::LintWarning;

impl LintMultiLineStatement {
    pub fn visit_ast_expr(&mut self, node: *mut AstExpr) -> bool {
        let node = unsafe { &*node };
        let top = self.stack.last_mut().unwrap();

        if !top.flagged {
            let location = node.base.location;

            if location.begin.line > top.lastLine() {
                top.lastLine = location.begin.line;

                if location.begin.column <= top.start.begin.column {
                    emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_MultiLineStatement,
                        location,
                        format_args!("Statement spans multiple lines; use indentation to silence"),
                    );

                    top.flagged = true;
                }
            }
        }

        true
    }
}
