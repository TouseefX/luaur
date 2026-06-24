use crate::records::pending_type_pack::PendingTypePack;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_pack_id::TypePackId;

impl TxnLog {
    pub fn queue_type_pack_id(&mut self, tp: TypePackId) -> *mut PendingTypePack {
        unsafe {
            if (*tp).persistent {
                self.radioactive = true;
            }

            if let Some(existing) = self.type_pack_changes.find_mut(&tp) {
                return existing.as_mut() as *mut PendingTypePack;
            }

            let mut pending = (*tp).clone();
            pending.owningArena = core::ptr::null_mut();
            let (entry, _) = self
                .type_pack_changes
                .try_insert(tp, Box::new(PendingTypePack { pending }));

            entry.as_mut() as *mut PendingTypePack
        }
    }
}
