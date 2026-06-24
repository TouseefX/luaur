use crate::records::unifier::Unifier;

impl Unifier {
    pub fn ice_string(&mut self, message: &str) {
        self.ice_string_location(message, &luaur_ast::records::location::Location::default());
    }
}
