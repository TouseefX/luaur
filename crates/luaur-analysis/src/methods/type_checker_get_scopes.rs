use crate::records::type_checker::TypeChecker;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;

impl TypeChecker {
    pub fn get_scopes(&self) -> Vec<(Location, ScopePtr)> {
        Vec::new()
    }
}
