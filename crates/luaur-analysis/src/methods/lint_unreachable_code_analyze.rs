use crate::enums::status::Status;
use crate::functions::does_call_error::does_call_error;
use crate::functions::emit_warning::emit_warning;
use crate::records::lint_unreachable_code::LintUnreachableCode;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_stat_break::AstStatBreak;
use luaur_ast::records::ast_stat_continue::AstStatContinue;
use luaur_ast::records::ast_stat_expr::AstStatExpr;
use luaur_ast::records::ast_stat_for::AstStatFor;
use luaur_ast::records::ast_stat_for_in::AstStatForIn;
use luaur_ast::records::ast_stat_if::AstStatIf;
use luaur_ast::records::ast_stat_repeat::AstStatRepeat;
use luaur_ast::records::ast_stat_return::AstStatReturn;
use luaur_ast::records::ast_stat_while::AstStatWhile;
use luaur_config::enums::code::Code;

impl LintUnreachableCode {
    pub fn analyze(&mut self, node: *mut AstStat) -> Status {
        if node.is_null() {
            return Status::Unknown;
        }

        let node_base = node as *mut AstNode;

        let stat = unsafe { luaur_ast::rtti::ast_node_as::<AstStatBlock>(node_base) };
        if !stat.is_null() {
            let body = unsafe { (*stat).body };

            for i in 0..body.size {
                let si = unsafe { *body.data.add(i) };
                let step = self.analyze(si);

                if step != Status::Unknown {
                    if i + 1 == body.size {
                        return step;
                    }

                    let next = unsafe { *body.data.add(i + 1) };

                    if step == Status::Error
                        && unsafe {
                            luaur_ast::rtti::ast_node_is::<AstStatExpr>(&(*si).base)
                                && luaur_ast::rtti::ast_node_is::<AstStatReturn>(&(*next).base)
                        }
                        && i + 2 == body.size
                    {
                        return Status::Error;
                    }

                    emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_UnreachableCode,
                        unsafe { (*next).base.location },
                        format_args!(
                            "Unreachable code (previous statement always {}s)",
                            self.get_reason(step)
                        ),
                    );
                    return step;
                }
            }

            return Status::Unknown;
        }

        let stat = unsafe { luaur_ast::rtti::ast_node_as::<AstStatIf>(node_base) };
        if !stat.is_null() {
            let ifs = self.analyze(unsafe { (*stat).thenbody as *mut AstStat });
            let elses = unsafe {
                if (*stat).elsebody.is_null() {
                    Status::Unknown
                } else {
                    self.analyze((*stat).elsebody)
                }
            };
            return min_status(ifs, elses);
        }

        let stat = unsafe { luaur_ast::rtti::ast_node_as::<AstStatWhile>(node_base) };
        if !stat.is_null() {
            self.analyze(unsafe { (*stat).body as *mut AstStat });
            return Status::Unknown;
        }

        let stat = unsafe { luaur_ast::rtti::ast_node_as::<AstStatRepeat>(node_base) };
        if !stat.is_null() {
            self.analyze(unsafe { (*stat).body as *mut AstStat });
            return Status::Unknown;
        }

        if unsafe { luaur_ast::rtti::ast_node_is::<AstStatBreak>(node_base) } {
            return Status::Break;
        }

        if unsafe { luaur_ast::rtti::ast_node_is::<AstStatContinue>(node_base) } {
            return Status::Continue;
        }

        if unsafe { luaur_ast::rtti::ast_node_is::<AstStatReturn>(node_base) } {
            return Status::Return;
        }

        let stat = unsafe { luaur_ast::rtti::ast_node_as::<AstStatExpr>(node_base) };
        if !stat.is_null() {
            let expr = unsafe { (*stat).expr };
            let call = unsafe { luaur_ast::rtti::ast_node_as::<AstExprCall>(expr as *mut AstNode) };

            if !call.is_null() && unsafe { does_call_error(&*call) } {
                return Status::Error;
            }

            return Status::Unknown;
        }

        let stat = unsafe { luaur_ast::rtti::ast_node_as::<AstStatFor>(node_base) };
        if !stat.is_null() {
            self.analyze(unsafe { (*stat).body as *mut AstStat });
            return Status::Unknown;
        }

        let stat = unsafe { luaur_ast::rtti::ast_node_as::<AstStatForIn>(node_base) };
        if !stat.is_null() {
            self.analyze(unsafe { (*stat).body as *mut AstStat });
            return Status::Unknown;
        }

        Status::Unknown
    }
}

fn min_status(lhs: Status, rhs: Status) -> Status {
    if status_rank(lhs) <= status_rank(rhs) {
        lhs
    } else {
        rhs
    }
}

fn status_rank(status: Status) -> u8 {
    match status {
        Status::Unknown => 0,
        Status::Continue => 1,
        Status::Break => 2,
        Status::Return => 3,
        Status::Error => 4,
    }
}
