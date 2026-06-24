use crate::records::traversal_state::TraversalState;
use luaur_common::DFInt;

impl TraversalState {
    pub fn too_long(&mut self) -> bool {
        self.steps += 1;
        self.steps > DFInt::LuauTypePathMaximumTraverseSteps.get()
    }
}
