use luaur_ast::records::ast_stat_for_in::AstStatForIn;

impl crate::records::type_attacher::TypeAttacher {
    pub fn visit_ast_stat_for_in(&mut self, stat: *mut AstStatForIn) -> bool {
        let stat_ref = unsafe { &*stat };
        for i in 0..stat_ref.vars.size {
            let var = unsafe { *stat_ref.vars.data.add(i) };
            self.visit_local(var);
        }
        true
    }
}
