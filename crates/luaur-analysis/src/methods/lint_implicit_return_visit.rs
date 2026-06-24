use crate::functions::emit_warning::emit_warning;
use crate::functions::get_fallthrough::get_fallthrough;
use crate::records::lint_implicit_return::LintImplicitReturn;
use core::ffi::CStr;
use luaur_ast::records::ast_expr_function::AstExprFunction;
use luaur_ast::records::ast_stat::AstStat;
use luaur_config::enums::code::Code;

impl LintImplicitReturn {
    pub fn visit(&mut self, node: *mut AstExprFunction) -> bool {
        let node = unsafe { &*node };
        let bodyf = get_fallthrough(node.body as *const AstStat);
        let vret = self.get_value_return(node.body as *mut core::ffi::c_void);

        if !bodyf.is_null() && !vret.is_null() {
            let location = self.get_end_location(bodyf as *const core::ffi::c_void);
            let return_line = unsafe { (*vret).base.base.location.begin.line + 1 };
            let context = unsafe { &mut *self.context };

            if !node.debugname.value.is_null() {
                let debugname = unsafe { CStr::from_ptr(node.debugname.value) }.to_string_lossy();
                emit_warning(
                    context,
                    Code::Code_ImplicitReturn,
                    location,
                    format_args!(
                        "Function '{}' can implicitly return no values even though there's an explicit return at line {}; add explicit return to silence",
                        debugname,
                        return_line
                    ),
                );
            } else {
                emit_warning(
                    context,
                    Code::Code_ImplicitReturn,
                    location,
                    format_args!(
                        "Function can implicitly return no values even though there's an explicit return at line {}; add explicit return to silence",
                        return_line
                    ),
                );
            }
        }

        true
    }
}
