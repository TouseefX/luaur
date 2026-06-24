use crate::records::substitution::Substitution;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Substitution {
    pub fn substitution_txn_log_type_arena(&mut self, log_: *const TxnLog, arena: *mut TypeArena) {
        self.arena = arena;
        self.base.log = log_;
        LUAU_ASSERT!(!log_.is_null());
    }
}
