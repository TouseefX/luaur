use crate::records::lint_global_local::LintGlobalLocal;
use luaur_ast::records::ast_stat_function::AstStatFunction;
use luaur_config::enums::code::Code;

impl LintGlobalLocal {
    pub fn visit_ast_stat_function(&mut self, node: *mut AstStatFunction) -> bool {
        let name = unsafe { (*node).name };
        if unsafe {
            luaur_ast::rtti::ast_node_is::<luaur_ast::records::ast_expr_global::AstExprGlobal>(
                &(*name).base,
            )
        } {
            let gv = name as *mut luaur_ast::records::ast_expr_global::AstExprGlobal;
            let g = self.globals.get_or_insert(unsafe { (*gv).name });

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
                g.definedAsFunction = true;
                g.definedInModuleScope = self.function_stack.is_empty();
            }

            self.track_global_ref(gv);
        }

        true
    }
}

fn name_str(value: *const core::ffi::c_char) -> alloc::borrow::Cow<'static, str> {
    if value.is_null() {
        alloc::borrow::Cow::Borrowed("")
    } else {
        unsafe { core::ffi::CStr::from_ptr(value).to_string_lossy() }
    }
}
