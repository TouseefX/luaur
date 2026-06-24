use crate::records::lint_global_local::LintGlobalLocal;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_config::enums::code::Code;

impl LintGlobalLocal {
    pub fn visit_ast_expr_global(&mut self, node: *mut AstExprGlobal) -> bool {
        if !self.function_stack.is_empty()
            && !self
                .function_stack
                .last()
                .unwrap()
                .dominated_globals
                .contains(unsafe { &(*node).name })
        {
            let g = self.globals.get_or_insert(unsafe { (*node).name });
            g.readBeforeWritten = true;
        }

        self.track_global_ref(node);

        if unsafe { (*node).name } == unsafe { (*self.context).placeholder } {
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
