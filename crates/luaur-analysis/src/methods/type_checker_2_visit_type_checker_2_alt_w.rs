use crate::records::type_checker_2::TypeChecker2;
use luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType;

impl TypeChecker2 {
    pub fn visit_ast_stat_declare_extern_type(&mut self, stat: *mut AstStatDeclareExternType) {
        unsafe {
            let stat_ref = &*stat;
            for i in 0..stat_ref.props.size {
                let prop = &*stat_ref.props.data.add(i);
                self.visit_ast_type(prop.ty);
            }
        }
    }
}
