use crate::records::lint_local_hygiene::LintLocalHygiene;
use luaur_ast::records::ast_type_reference::AstTypeReference;

impl LintLocalHygiene {
    pub fn visit_ast_type_reference(&mut self, node: *mut AstTypeReference) -> bool {
        let node_ref = unsafe { &*node };
        let Some(prefix) = node_ref.prefix else {
            return true;
        };

        if let Some(ast_local) = self.imports.find(&prefix) {
            let local = self.locals.get_or_insert(*ast_local);
            debug_assert!(local.import);
            local.used = true;
        }

        true
    }
}
