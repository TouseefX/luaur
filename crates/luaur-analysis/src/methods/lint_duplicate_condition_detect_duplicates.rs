use crate::records::lint_duplicate_condition::LintDuplicateCondition;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_config::enums::code::Code;

impl LintDuplicateCondition {
    pub fn detect_duplicates(&mut self, conditions: &Vec<*mut AstExpr>) {
        const K_MAX_DISTANCE: usize = 5;

        for i in 0..conditions.len() {
            let start = i.saturating_sub(K_MAX_DISTANCE);

            for j in start..i {
                if crate::functions::similar::similar(conditions[j], conditions[i]) {
                    let current = unsafe { (*conditions[i]).base.location };
                    let previous = unsafe { (*conditions[j]).base.location };

                    if current.begin.line == previous.begin.line {
                        crate::functions::emit_warning::emit_warning(
                            unsafe { &mut *self.context },
                            Code::Code_DuplicateCondition,
                            current,
                            format_args!(
                                "Condition has already been checked on column {}",
                                previous.begin.column + 1
                            ),
                        );
                    } else {
                        crate::functions::emit_warning::emit_warning(
                            unsafe { &mut *self.context },
                            Code::Code_DuplicateCondition,
                            current,
                            format_args!(
                                "Condition has already been checked on line {}",
                                previous.begin.line + 1
                            ),
                        );
                    }

                    break;
                }
            }
        }
    }
}
