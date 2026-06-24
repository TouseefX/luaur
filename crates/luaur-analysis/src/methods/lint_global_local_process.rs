use crate::records::lint_context::LintContext;
use crate::records::lint_global_local::LintGlobalLocal;

impl LintGlobalLocal {
    pub fn process(context: &mut LintContext) {
        let mut pass = LintGlobalLocal::lint_global_local();
        pass.context = context as *mut LintContext;

        for (name, global) in context.builtin_globals.iter() {
            let g = pass.globals.get_or_insert(*name);
            g.builtin = true;
            g.deprecated = global.deprecated;
        }

        unsafe {
            let root = (*pass.context).root;
            luaur_ast::visit::ast_stat_visit(root, &mut pass);
        }

        pass.report();
    }
}
