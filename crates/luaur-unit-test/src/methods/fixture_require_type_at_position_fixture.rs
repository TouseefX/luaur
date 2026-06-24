use crate::records::fixture::Fixture;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_ast::records::position::Position;

impl Fixture {
    pub fn require_type_at_position_position(&mut self, position: Position) -> TypeId {
        let ty = self.find_type_at_position_module_name_position("".as_ref(), position);
        if let Some(ty) = ty {
            ty
        } else {
            luaur_common::LUAU_ASSERT!(false);
            core::ptr::null_mut()
        }
    }
}
