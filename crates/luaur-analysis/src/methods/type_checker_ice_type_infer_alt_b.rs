use crate::records::type_checker::TypeChecker;

impl TypeChecker {
    pub fn ice_string(&mut self, message: &str) {
        self.ice_string_location(message, &luaur_ast::records::location::Location::default());
    }
}
