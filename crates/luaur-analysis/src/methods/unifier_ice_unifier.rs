use crate::records::unifier::Unifier;
use luaur_ast::records::location::Location;

impl Unifier {
    pub fn ice_string_location(&mut self, message: &str, location: &Location) {
        self.ice_string(message);
        let _ = location;
    }
}
