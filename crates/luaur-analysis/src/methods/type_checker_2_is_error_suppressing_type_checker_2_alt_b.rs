use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_id::TypeId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn is_error_suppressing_location_type_id_location_type_id(
        &mut self,
        loc1: Location,
        ty1: TypeId,
        loc2: Location,
        ty2: TypeId,
    ) -> bool {
        self.is_error_suppressing_location_type_id(loc1, ty1)
            || self.is_error_suppressing_location_type_id(loc2, ty2)
    }
}
