use crate::records::build_queue_item::BuildQueueItem;
use crate::records::frontend::Frontend;
use alloc::vec::Vec;

impl Frontend {
    pub fn check_build_queue_items(&mut self, items: &mut Vec<BuildQueueItem>) {
        for item in items.iter_mut() {
            self.check_build_queue_item(item);

            if item.module.cancelled {
                break;
            }

            self.record_item_result(item);
        }
    }
}
