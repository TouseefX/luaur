use crate::records::tarjan::Tarjan;
use crate::records::tarjan_node::TarjanNode;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;

impl Tarjan {
    pub fn indexify_type_id(&mut self, ty: TypeId) -> (i32, bool) {
        let ty = unsafe { (*self.log).follow_type_id(ty) };

        if let Some(&index) = self.type_to_index.find(&ty) {
            (index, false)
        } else {
            let index = self.nodes.len() as i32;
            self.type_to_index.try_insert(ty, index);
            self.nodes.push(TarjanNode {
                ty,
                tp: std::ptr::null_mut(),
                on_stack: false,
                dirty: false,
                lowlink: index,
            });
            (index, true)
        }
    }
}
