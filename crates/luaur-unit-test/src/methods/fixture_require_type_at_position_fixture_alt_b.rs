use crate::records::fixture::Fixture;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_ast::records::position::Position;

impl Fixture {
    pub fn require_type_at_position_module_name_position(
        &mut self,
        module_name: &str,
        position: Position,
    ) -> TypeId {
        let ty = self.find_type_at_position_module_name_position(module_name, position);
        luaur_common::LUAU_ASSERT!(ty.is_some());
        match ty {
            Some(ty) => ty,
            None => {
                // Mirror the C++ contract: assert that the type must exist.
                // Return a null pointer on mismatch to keep this function total.
                core::ptr::null_mut()
            }
        }
    }
}
