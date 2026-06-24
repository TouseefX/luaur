use crate::records::state_dot::StateDot;
use luaur_common::functions::format_append::formatAppend;

impl StateDot {
    pub fn start_node(&mut self, index: i32) {
        formatAppend(&mut self.result, format_args!("n{} [", index));
    }
}
