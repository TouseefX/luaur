use crate::records::builtin_types::BuiltinTypes;
use crate::records::replace_generics::ReplaceGenerics;
use crate::records::scope::Scope;
use crate::records::substitution::Substitution;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::records::type_level::TypeLevel;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

impl ReplaceGenerics {
    pub fn reset_state(
        &mut self,
        log: *const TxnLog,
        arena: *mut TypeArena,
        builtin_types: *mut BuiltinTypes,
        level: TypeLevel,
        scope: *mut Scope,
        generics: Vec<TypeId>,
        generic_packs: Vec<TypePackId>,
    ) {
        self.base.reset_state(log, arena);

        self.builtin_types = builtin_types;

        self.level = level;
        self.scope = scope;

        self.generics = generics;
        self.generic_packs = generic_packs;
    }
}
