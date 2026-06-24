use crate::records::type_checker::TypeChecker;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn ice_string_location(&mut self, message: &str, location: &Location) {
        self.ice_string(message);
        let _ = location;
    }
}
