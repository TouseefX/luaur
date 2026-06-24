use crate::records::lint_global_local::LintGlobalLocal;
use luaur_ast::records::ast_expr_global::AstExprGlobal;

impl LintGlobalLocal {
    pub fn track_global_ref(&mut self, node: *mut AstExprGlobal) {
        let current_function_refs = self
            .function_stack
            .iter()
            .map(|entry| entry.ast)
            .collect::<alloc::vec::Vec<_>>();

        self.global_refs.push(node);

        let g = self.globals.get_or_insert(unsafe { (*node).name });

        if g.firstRef.is_null() {
            g.firstRef = node;

            if !g.builtin {
                g.functionRef.clear();
                g.functionRef.reserve(current_function_refs.len());
                g.functionRef.extend(current_function_refs);
            }
        } else if !g.builtin {
            let mut prefix = 0;

            while prefix < g.functionRef.len()
                && prefix < current_function_refs.len()
                && g.functionRef[prefix] == current_function_refs[prefix]
            {
                prefix += 1;
            }

            g.functionRef.truncate(prefix);
        }
    }
}
