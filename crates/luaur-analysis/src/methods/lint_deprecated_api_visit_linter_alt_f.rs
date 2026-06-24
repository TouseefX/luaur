use crate::functions::emit_warning::emit_warning;
use crate::functions::is_number::is_number;
use crate::records::lint_deprecated_api::LintDeprecatedApi;
use luaur_ast::records::ast_expr_call::AstExprCall;
use luaur_ast::records::ast_expr_constant_number::AstExprConstantNumber;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_node::AstNode;
use luaur_config::enums::code::Code;

impl LintDeprecatedApi {
    pub fn visit_ast_expr_call(&mut self, node: *mut AstExprCall) -> bool {
        unsafe {
            if (*node).self_ || (*node).args.size < 1 {
                return true;
            }

            let fenv = luaur_ast::rtti::ast_node_as::<AstExprGlobal>((*node).func as *mut AstNode);

            if fenv.is_null()
                || !((*fenv).name.operator_eq_c_char(c"getfenv".as_ptr())
                    || (*fenv).name.operator_eq_c_char(c"setfenv".as_ptr()))
            {
                return true;
            }

            let level = *(*node).args.data.add(0);
            let ty = (*self.context).get_type(level);
            let level_is_number = ty.is_some_and(is_number)
                || (*(level as *mut AstNode)).is::<AstExprConstantNumber>();

            if level_is_number {
                let suggestion = if (*fenv).name.operator_eq_c_char(c"getfenv".as_ptr()) {
                    "; consider using 'debug.info' instead"
                } else {
                    ""
                };
                let function_name = core::ffi::CStr::from_ptr((*fenv).name.value).to_string_lossy();

                emit_warning(
                    &mut *self.context,
                    Code::Code_DeprecatedApi,
                    (*node).base.base.location,
                    format_args!("Function '{}' is deprecated{}", function_name, suggestion),
                );
            }
        }

        true
    }
}
