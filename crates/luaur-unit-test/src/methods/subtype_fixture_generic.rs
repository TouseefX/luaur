use crate::records::subtype_fixture::SubtypeFixture;
use alloc::string::ToString;
use alloc::sync::Arc;
use luaur_analysis::records::generic_type::GenericType;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::type_aliases::type_id::TypeId;

impl SubtypeFixture {
    pub fn generic(&mut self, name: &str) -> TypeId {
        let scope = Arc::as_ptr(&self.module_scope) as *mut Scope;
        self.arena.add_type(GenericType::generic_type_scope_name(
            scope,
            &name.to_string(),
        ))
    }
}
