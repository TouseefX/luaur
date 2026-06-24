use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl LintLocalHygiene {
    pub fn visit_ast_stat_local(&mut self, node: *mut AstStatLocal) -> bool {
        let vars = unsafe { (*node).vars };
        let values = unsafe { (*node).values };

        if vars.size == 1 && values.size == 1 {
            let local = unsafe { *vars.data };
            let value = unsafe { *values.data };
            let is_import = self.is_require_call(value);

            {
                let info = self.locals.get_or_insert(local);
                info.defined = node.cast();
                info.import = is_import;
            }

            if is_import {
                *self.imports.get_or_insert(unsafe { (*local).name }) = local;
            }
        } else {
            for i in 0..vars.size {
                let local = unsafe { *vars.data.add(i) };
                let info = self.locals.get_or_insert(local);
                info.defined = node.cast();
            }
        }

        true
    }
}
