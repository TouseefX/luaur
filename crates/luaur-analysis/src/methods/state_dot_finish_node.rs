use crate::records::state_dot::StateDot;
use luaur_common::functions::format_append::formatAppend;

impl StateDot {
    pub fn finish_node(&mut self) {
        formatAppend(&mut self.result, format_args!("];\n"));
    }
}
