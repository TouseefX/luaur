use crate::records::scope::Scope;
use crate::type_aliases::scope_ptr_scope::ScopePtr;

impl Scope {
    pub fn inherit_assignments(&mut self, child_scope: &ScopePtr) {
        for (k, a) in unsafe { &(*child_scope).lvalue_types }.iter() {
            self.lvalue_types.try_insert(*k, *a);
        }
    }
}
