use crate::records::state_dot::StateDot;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::functions::format_append::formatAppend;

impl StateDot {
    pub fn finish_node_label_type_pack_id(&mut self, tp: TypePackId) {
        if self.opts.show_pointers {
            formatAppend(&mut self.result, format_args!("\n0x{:p}", tp));
        }
        self.result += "\"";
    }
}
