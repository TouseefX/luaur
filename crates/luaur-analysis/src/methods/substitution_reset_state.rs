use crate::records::substitution::Substitution;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl Substitution {
    pub fn reset_state(&mut self, log: *const TxnLog, arena: *mut TypeArena) {
        self.base.clear_tarjan(log);

        self.arena = arena;

        self.new_types.clear();
        self.new_packs.clear();
        self.replaced_types.clear();
        self.replaced_type_packs.clear();

        self.no_traverse_types.clear();
        self.no_traverse_type_packs.clear();
    }
}
