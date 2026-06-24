use crate::records::state_dot::StateDot;
use crate::records::to_dot_options::ToDotOptions;

impl StateDot {
    pub fn state_dot_state_dot(opts: ToDotOptions) -> Self {
        Self::new(opts)
    }
}
