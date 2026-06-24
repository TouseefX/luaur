use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint_graph::ConstraintGraph;
use core::ptr::NonNull;

impl ConstraintGraph {
    pub fn constraint_graph(&mut self, builtin_types: NonNull<BuiltinTypes>) {
        self.builtin_types = builtin_types;
    }
}
