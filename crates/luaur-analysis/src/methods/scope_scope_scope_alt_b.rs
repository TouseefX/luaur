use crate::records::scope::Scope;
use crate::type_aliases::scope_ptr_scope::ScopePtr;

impl Scope {
    pub fn scope_scope_ptr_i32(&mut self, parent: &ScopePtr, sub_level: i32) {
        self.parent = Some(parent.clone());
        self.return_type = parent.return_type.clone();
        self.level = parent.level.incr();
        self.level.subLevel = sub_level;
    }
}
