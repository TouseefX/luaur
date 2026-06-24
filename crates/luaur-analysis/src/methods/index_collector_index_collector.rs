use crate::records::index_collector::IndexCollector;
use crate::records::type_arena::TypeArena;
use crate::records::type_once_visitor::TypeOnceVisitor;
use alloc::string::ToString;

impl IndexCollector {
    pub fn index_collector(&mut self, arena: *mut TypeArena) {
        self.base = TypeOnceVisitor::new("IndexCollector".to_string(), true);
        self.arena = arena;
    }
}
