use crate::records::find_nth_occurence_of::FindNthOccurenceOf;
use luaur_ast::records::ast_type::AstType;

impl FindNthOccurenceOf {
    pub fn visit_ast_type(&mut self, t: *mut AstType) -> bool {
        unsafe {
            luaur_ast::visit::ast_type_visit(t, self);
        }
        !self.the_node.is_null()
    }
}
