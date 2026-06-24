use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use crate::type_aliases::type_pack_id::TypePackId;

impl IterativeTypeVisitor {
    pub fn iterative_type_visitor_traverse(&mut self) {
        self.traverse_type_pack_id(core::ptr::null());
    }

    pub fn traverse_type_pack_id(&mut self, tp: TypePackId) {
        self.work_queue.push(
            crate::records::work_item_iterative_type_visitor::WorkItem::work_item_type_pack_id_i32(
                tp,
                self.parent_cursor,
            ),
        );
    }
}
