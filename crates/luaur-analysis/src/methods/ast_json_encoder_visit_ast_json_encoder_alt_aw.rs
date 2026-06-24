use crate::macros::prop::prop;
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_type_intersection::AstTypeIntersection;

impl AstJsonEncoder {
    pub fn visit_ast_type_intersection(&mut self, node: *mut AstTypeIntersection) -> bool {
        self.write_ast_type_intersection(node);
        false
    }
}
