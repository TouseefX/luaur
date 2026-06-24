use crate::records::scope::Scope;
use crate::type_aliases::name_type::Name;
use crate::type_aliases::type_pack_id::TypePackId;

impl Scope {
    pub fn lookup_pack(&self, name: &Name) -> Option<TypePackId> {
        let mut scope: &Scope = self;
        loop {
            if let Some(type_pack_id) = scope.private_type_pack_bindings.get(name) {
                return Some(*type_pack_id);
            }

            if let Some(parent) = scope.parent.as_ref() {
                scope = parent.as_ref();
            } else {
                return None;
            }
        }
    }
}
