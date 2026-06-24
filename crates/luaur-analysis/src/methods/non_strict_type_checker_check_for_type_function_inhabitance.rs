use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl NonStrictTypeChecker {
    pub fn check_for_type_function_inhabitance(
        &mut self,
        _instance: TypeId,
        _location: Location,
    ) -> TypeId {
        _instance
    }
}
