use crate::records::build_queue_work_state::BuildQueueWorkState;
use crate::records::frontend::Frontend;
use alloc::sync::Arc;

impl Frontend {
    pub fn send_queue_cycle_item_task(&mut self, state: Arc<BuildQueueWorkState>) {
        let s = unsafe { &*(&*state as *const BuildQueueWorkState) };
        for i in 0..s.build_queue_items.len() {
            if !s.build_queue_items[i].processing {
                self.send_queue_item_tasks(state.clone(), vec![i]);
                break;
            }
        }
    }
}
