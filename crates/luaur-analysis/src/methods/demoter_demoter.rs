use crate::records::builtin_types::BuiltinTypes;
use crate::records::demoter::Demoter;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;

impl Demoter {
    pub fn demoter(&mut self, arena: *mut TypeArena, builtins: *mut BuiltinTypes) {
        self.arena = arena;
        self.builtins = builtins;
    }
}
