use crate::records::tracing_visitor::TracingVisitor;
use luaur_analysis::type_aliases::type_id::TypeId;

impl TracingVisitor {
    pub fn cycle_type_id(&mut self, ty: TypeId) {
        self.cycles.push(ty);
    }
}
