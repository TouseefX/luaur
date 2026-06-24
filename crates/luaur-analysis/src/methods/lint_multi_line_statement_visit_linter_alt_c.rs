use crate::records::lint_multi_line_statement::LintMultiLineStatement;

use luaur_ast::records::ast_stat_repeat::AstStatRepeat;

use core::ffi::c_void;

impl LintMultiLineStatement {
    pub fn visit_ast_stat_repeat(&mut self, node: *mut AstStatRepeat) -> bool {
        let node_body = unsafe { (*node).body };
        self.visit_ast_stat_block(node_body);
        let _ = unsafe { node as *mut c_void };
        false
    }
}
