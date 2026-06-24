use crate::records::lint_global_local::LintGlobalLocal;
use luaur_ast::records::ast_expr_local::AstExprLocal;
use luaur_config::enums::code::Code;

impl LintGlobalLocal {
    pub fn visit_ast_expr_local(&mut self, node: *mut AstExprLocal) -> bool {
        let local = unsafe { (*node).local };
        if !local.is_null() && unsafe { (*local).name } == unsafe { (*self.context).placeholder } {
            crate::functions::emit_warning::emit_warning(
                unsafe { &mut *self.context },
                Code::Code_PlaceholderRead,
                unsafe { (*node).base.base.location },
                format_args!("Placeholder value '_' is read here; consider using a named variable"),
            );
        }

        true
    }
}
