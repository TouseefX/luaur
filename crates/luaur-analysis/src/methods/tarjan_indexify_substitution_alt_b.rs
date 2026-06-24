use crate::records::tarjan::Tarjan;
use crate::type_aliases::type_pack_id::TypePackId;

impl Tarjan {
    pub fn indexify_type_pack_id(&mut self, mut tp: TypePackId) -> (i32, bool) {
        tp = unsafe { (*self.log).follow_type_pack_id(tp) };

        if let Some(&index) = self.pack_to_index.find(&tp) {
            (index, false)
        } else {
            let index = self.nodes.len() as i32;

            self.pack_to_index.try_insert(tp, index);

            self.nodes.push(crate::records::tarjan_node::TarjanNode {
                ty: core::ptr::null_mut(),
                tp,
                on_stack: false,
                dirty: false,
                lowlink: index,
            });

            (index, true)
        }
    }
}
