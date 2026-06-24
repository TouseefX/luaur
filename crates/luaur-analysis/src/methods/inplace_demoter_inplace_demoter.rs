use crate::records::inplace_demoter::InplaceDemoter;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::records::type_once_visitor::TypeOnceVisitor;

impl InplaceDemoter {
    pub fn inplace_demoter(&mut self, level: TypeLevel, arena: *mut TypeArena) {
        self.new_level = level;
        self.arena = arena;
    }
}
