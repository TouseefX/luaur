use crate::records::lint_global_local::LintGlobalLocal;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_config::enums::code::Code;

impl LintGlobalLocal {
    pub fn visit_ast_stat_assign(&mut self, node: *mut AstStatAssign) -> bool {
        let vars = unsafe { (*node).vars };
        for i in 0..vars.size {
            let var = unsafe { *vars.data.add(i) };

            if unsafe {
                luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_expr_global::AstExprGlobal>(
                    &(*var).base,
                )
            } {
                let gv = var as *mut luaur_ast::records::ast_expr_global::AstExprGlobal;
                let g = self.globals.get_or_insert(unsafe { (*gv).name });

                if self.function_stack.is_empty() {
                    g.definedInModuleScope = true;
                } else if !self.function_stack.last().unwrap().conditional_execution {
                    self.function_stack
                        .last_mut()
                        .unwrap()
                        .dominated_globals
                        .insert(unsafe { (*gv).name });
                }

                if g.builtin {
                    crate::functions::emit_warning::emit_warning(
                        unsafe { &mut *self.context },
                        Code::Code_BuiltinGlobalWrite,
                        unsafe { (*gv).base.base.location },
                        format_args!(
                            "Built-in global '{}' is overwritten here; consider using a local or changing the name",
                            name_str(unsafe { (*gv).name.value })
                        ),
                    );
                } else {
                    g.assigned = true;
                }

                self.track_global_ref(gv);
            } else if unsafe {
                luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_expr_local::AstExprLocal>(
                    &(*var).base,
                )
            } {
                // Local writes are not local reads.
            } else {
                unsafe {
                    luaur_ast::visit::ast_expr_visit(var, self);
                }
            }
        }

        let values = unsafe { (*node).values };
        for i in 0..values.size {
            unsafe {
                luaur_ast::visit::ast_expr_visit(*values.data.add(i), self);
            }
        }

        false
    }
}

fn name_str(value: *const core::ffi::c_char) -> alloc::borrow::Cow<'static, str> {
    if value.is_null() {
        alloc::borrow::Cow::Borrowed("")
    } else {
        unsafe { core::ffi::CStr::from_ptr(value).to_string_lossy() }
    }
}
