use crate::functions::lookup_name::lookup_name;
use crate::records::fixture::Fixture;
use alloc::string::String;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::type_aliases::type_id::TypeId;

impl Fixture {
    pub fn require_type_scope_ptr_string(&mut self, scope: *mut Scope, name: &String) -> TypeId {
        let ty = lookup_name(scope, name);
        if ty.is_none() {
            panic!("requireType: No type \"{}\"", name);
        }
        ty.unwrap()
    }
}
