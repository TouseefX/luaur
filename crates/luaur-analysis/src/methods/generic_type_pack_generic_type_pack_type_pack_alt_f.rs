use crate::records::generic_type_pack::GenericTypePack;
use crate::records::scope::Scope;
use crate::type_aliases::name_type::Name;

impl GenericTypePack {
    pub fn generic_type_pack_scope_name(&mut self, _scope: *mut Scope, _name: &Name) {
        self.generic_type_pack();
        self.generic_type_pack_name(_name);
    }
}
