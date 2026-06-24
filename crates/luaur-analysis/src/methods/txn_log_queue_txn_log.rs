use crate::records::pending_type::PendingType;
use crate::records::txn_log::TxnLog;
use crate::type_aliases::type_id::TypeId;

impl TxnLog {
    pub fn queue_type_id(&mut self, ty: TypeId) -> *mut PendingType {
        unsafe {
            if (*ty).persistent {
                self.radioactive = true;
            }

            if let Some(existing) = self.type_var_changes.find_mut(&ty) {
                if !existing.dead {
                    return existing.as_mut() as *mut PendingType;
                }

                let mut pending = (*ty).clone();
                pending.owning_arena = core::ptr::null_mut();
                *existing = Box::new(PendingType {
                    pending,
                    dead: false,
                });
                return existing.as_mut() as *mut PendingType;
            }

            let mut pending = (*ty).clone();
            pending.owning_arena = core::ptr::null_mut();
            let (entry, _) = self.type_var_changes.try_insert(
                ty,
                Box::new(PendingType {
                    pending,
                    dead: false,
                }),
            );

            entry.as_mut() as *mut PendingType
        }
    }
}
