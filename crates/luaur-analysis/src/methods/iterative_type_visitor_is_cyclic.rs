use crate::records::iterative_type_visitor::IterativeTypeVisitor;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl IterativeTypeVisitor {
    pub fn iterative_type_visitor_is_cyclic(&mut self) -> bool {
        let mut cursor = self.work_cursor as i32;

        while self.work_queue[self.work_cursor as usize].parent >= 0 {
            let item = &self.work_queue[self.work_cursor as usize];
            LUAU_ASSERT!(item.parent < cursor);
            cursor = item.parent;

            let item_ref = &self.work_queue[cursor as usize];

            if let Some(ty) = item.type_id() {
                if item_ref.operator_eq_type_id(ty) {
                    return true;
                }
            } else if let Some(tp) = item.type_pack_id() {
                if item_ref.operator_eq_type_pack_id(tp) {
                    return true;
                }
            }
        }

        false
    }
}
