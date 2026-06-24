use crate::records::state_dot::StateDot;
use crate::type_aliases::type_id::TypeId;
use luaur_common::functions::format_append::formatAppend;

impl StateDot {
    pub fn finish_node_label_type_id(&mut self, ty: TypeId) {
        if self.opts.show_pointers {
            formatAppend(&mut self.result, format_args!("\n0x{:p}", ty));
        }
        self.result += "\"";
    }
}
