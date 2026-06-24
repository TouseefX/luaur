use crate::records::type_checker_2::TypeChecker2;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;

impl TypeChecker2 {
    pub fn is_error_suppressing_location_type_pack_id_location_type_pack_id(
        &mut self,
        loc1: Location,
        tp1: TypePackId,
        loc2: Location,
        tp2: TypePackId,
    ) -> bool {
        self.is_error_suppressing_location_type_pack_id(loc1, tp1)
            || self.is_error_suppressing_location_type_pack_id(loc2, tp2)
    }
}
