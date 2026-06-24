use crate::records::internal_compiler_error::InternalCompilerError;
use crate::records::txn_log::TxnLog;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_iterator::TypePackIterator;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl TypePackIterator {
    pub fn operator_inc(&mut self) {
        LUAU_ASSERT!(!self.tp.is_null());

        self.currentIndex += 1;
        while !self.tp.is_null() && self.currentIndex >= unsafe { (*self.tp).head.len() } {
            self.currentTypePack = if let Some(tail) = unsafe { (*self.tp).tail } {
                unsafe { (*self.log).follow_type_pack_id(tail) }
            } else {
                core::ptr::null()
            };

            self.tp = if !self.currentTypePack.is_null() {
                unsafe { (*self.log).txn_log_get_mutable::<TypePack, _>(self.currentTypePack) }
            } else {
                core::ptr::null()
            };

            if !self.tp.is_null() {
                // Step twice on each iteration to detect cycles
                self.tailCycleCheck = if let Some(tail) = unsafe { (*self.tp).tail } {
                    unsafe { (*self.log).follow_type_pack_id(tail) }
                } else {
                    core::ptr::null()
                };

                if self.currentTypePack == self.tailCycleCheck {
                    panic!(
                        "{}",
                        InternalCompilerError::internal_compiler_error_string_string(
                            "TypePackIterator detected a type pack cycle".to_string(),
                            "".to_string(),
                        )
                        .message
                    );
                }
            }

            self.currentIndex = 0;
        }
    }
}
