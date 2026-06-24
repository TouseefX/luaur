use crate::records::constraint_list::ConstraintList;
use crate::type_aliases::constraint_vertex::ConstraintVertex;

impl ConstraintList {
    pub fn contains(&self, vertex: ConstraintVertex) -> bool {
        match self.present.find(&vertex) {
            Some(entry) => *entry,
            None => false,
        }
    }
}
