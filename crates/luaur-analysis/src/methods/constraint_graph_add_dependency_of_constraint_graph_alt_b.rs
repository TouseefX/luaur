use crate::records::constraint::Constraint;
use crate::records::constraint_graph::ConstraintGraph;
use luaur_common::records::variant::Variant3;

impl ConstraintGraph {
    pub fn add_dependency_of_constraint_constraint(
        &mut self,
        dependency: &mut Constraint,
        target: &mut Constraint,
    ) -> bool {
        let dep_vertex = Variant3::V2(dependency as *const Constraint);
        let target_vertex = Variant3::V2(target as *const Constraint);

        self.add_dependency_of_constraint_vertex_constraint_vertex(dep_vertex, target_vertex)
    }
}
