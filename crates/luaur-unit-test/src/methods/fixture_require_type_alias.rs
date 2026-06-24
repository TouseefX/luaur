use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::type_aliases::type_id::TypeId;

impl Fixture {
    pub fn require_type_alias(&mut self, name: &String) -> TypeId {
        let ty = self.lookup_type(name);
        let ty = ty.expect("type alias not found");
        unsafe { luaur_analysis::functions::follow_type::follow_type_id(ty) }
    }
}
