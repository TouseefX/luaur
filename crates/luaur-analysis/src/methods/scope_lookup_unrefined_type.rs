use crate::records::scope::Scope;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::type_id::TypeId;

impl Scope {
    pub fn lookup_unrefined_type(&self, def: DefId) -> Option<TypeId> {
        let mut current: Option<&Scope> = Some(self);
        while let Some(scope) = current {
            if let Some(ty) = scope.lvalue_types.find(&def) {
                return Some(*ty);
            }

            current = scope.parent.as_ref().map(|scoped| scoped.as_ref());
        }

        None
    }
}
