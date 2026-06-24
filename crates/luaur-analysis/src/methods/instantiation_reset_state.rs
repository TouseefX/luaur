use crate::records::builtin_types::BuiltinTypes;
use crate::records::instantiation::Instantiation;
use crate::records::scope::Scope;
use crate::records::substitution::Substitution;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;

impl Instantiation {
    pub fn reset_state(
        &mut self,
        log: *const TxnLog,
        arena: *mut TypeArena,
        builtin_types: *mut BuiltinTypes,
        level: TypeLevel,
        scope: *mut Scope,
    ) {
        Substitution::reset_state(&mut self.base, log, arena);

        self.builtin_types = builtin_types;
        self.level = level;
        self.scope = scope;

        self.reusable_replace_generics.reset_state(
            log,
            arena,
            builtin_types,
            level,
            scope,
            alloc::vec::Vec::new(),
            alloc::vec::Vec::new(),
        );
    }
}
