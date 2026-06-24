use crate::records::reduction::Reduction;
use crate::records::traversal_state::TraversalState;

impl TraversalState {
    pub fn traverse_type_path_reduction(&mut self, reduction: Reduction) -> bool {
        if self.check_invariants() {
            return false;
        }
        self.update_current_type_id(reduction.resultType);
        true
    }
}
