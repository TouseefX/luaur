use crate::functions::emit_warning::emit_warning;
use crate::records::lint_same_line_statement::LintSameLineStatement;
use core::ffi::c_void;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_local::AstStatLocal;
use luaur_config::enums::code::Code;

impl LintSameLineStatement {
    pub fn visit_stat_block(&mut self, node: *mut c_void) -> bool {
        let node = unsafe { &*node.cast::<AstStatBlock>() };
        let body = &node.body;

        for i in 1..body.size {
            let last_stmt = unsafe { *body.data.add((i - 1) as usize) };
            let current_stmt = unsafe { *body.data.add(i as usize) };

            let last = unsafe { (*last_stmt).base.location };
            let location = unsafe { (*current_stmt).base.location };

            if location.begin.line != last.end.line {
                continue;
            }

            if location.begin.line == self.last_line {
                continue;
            }

            let last_is_local =
                unsafe { luaur_ast::rtti::ast_node_is::<AstStatLocal>(&(*last_stmt).base) };
            let current_is_block =
                unsafe { luaur_ast::rtti::ast_node_is::<AstStatBlock>(&(*current_stmt).base) };

            if last_is_local && current_is_block {
                continue;
            }

            let last_has_semicolon = unsafe { (*last_stmt).has_semicolon };
            if last_has_semicolon {
                continue;
            }

            let context = unsafe { &mut *self.context };
            emit_warning(
                context,
                Code::Code_SameLineStatement,
                location,
                format_args!("A new statement is on the same line; add semi-colon on previous statement to silence"),
            );

            self.last_line = location.begin.line;
        }

        true
    }
}
