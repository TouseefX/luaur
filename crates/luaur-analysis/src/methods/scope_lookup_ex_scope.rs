use crate::records::def::Def;
use crate::records::scope::Scope;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::type_id::TypeId;

impl Scope {
    pub fn lookup_ex_def_id(&mut self, def: DefId) -> Option<(TypeId, *mut Scope)> {
        let mut s: *mut Scope = self as *mut Scope;

        loop {
            let lvalue_types = unsafe { &(*s).lvalue_types };
            if let Some(type_id) = lvalue_types.get(&def) {
                return Some((*type_id, s));
            }

            let rvalue_refinements = unsafe { &(*s).rvalue_refinements };
            if let Some(type_id) = rvalue_refinements.get(&def) {
                return Some((*type_id, s));
            }

            let parent = unsafe { &(*s).parent };
            match parent {
                Some(parent_scope) => {
                    s = parent_scope.as_ref() as *const Scope as *mut Scope;
                }
                None => {
                    return None;
                }
            }
        }
    }
}
