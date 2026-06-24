use crate::records::type_attacher::TypeAttacher;
use luaur_ast::records::ast_stat_local::AstStatLocal;

impl TypeAttacher {
    pub fn visit_ast_stat_local(&mut self, al: *mut AstStatLocal) -> bool {
        let al_ref = unsafe { &*al };

        for i in 0..al_ref.vars.size {
            let var = unsafe { *al_ref.vars.data.add(i) };
            self.visit_local(var);
        }

        true
    }
}
