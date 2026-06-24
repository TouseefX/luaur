impl crate::records::type_attacher::TypeAttacher {
    pub fn visit_ast_stat_for(
        &mut self,
        stat: *mut luaur_ast::records::ast_stat_for::AstStatFor,
    ) -> bool {
        let var = unsafe { (*stat).var };
        self.visit_local(var);
        true
    }
}
