use crate::records::binding::Binding;
use crate::records::scope::Scope;
use crate::records::symbol::Symbol;
use crate::type_aliases::type_id::TypeId;

impl Scope {
    pub fn lookup_symbol(&self, sym: Symbol) -> Option<TypeId> {
        let mut mutable_self = self as *const Scope as *mut Scope;
        let r = unsafe { (*mutable_self).lookup_ex_symbol(sym) };

        if let Some((binding, _)) = r {
            Some(unsafe { (*binding).type_id })
        } else {
            None
        }
    }
}
