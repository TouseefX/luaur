use crate::records::state_dot::StateDot;
use luaur_common::functions::format_append::formatAppend;

impl StateDot {
    pub fn start_node_label(&mut self) {
        formatAppend(&mut self.result, format_args!("label=\""));
    }
}
