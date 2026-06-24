use crate::records::replacer_deprecated::ReplacerDeprecated;
use crate::records::substitution::Substitution;
use crate::records::txn_log::TxnLog;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;
use luaur_common::records::dense_hash_map::DenseHashMap;

impl ReplacerDeprecated {
    pub fn replacer_deprecated(
        arena: *mut TypeArena,
        replacements: DenseHashMap<TypeId, TypeId>,
        replacement_packs: DenseHashMap<TypePackId, TypePackId>,
    ) -> Self {
        let mut base = Substitution {
            base: unsafe { core::mem::zeroed() },
            arena: core::ptr::null_mut(),
            new_types: DenseHashMap::new(core::ptr::null_mut()),
            new_packs: DenseHashMap::new(core::ptr::null_mut()),
            replaced_types: unsafe { core::mem::zeroed() },
            replaced_type_packs: unsafe { core::mem::zeroed() },
            no_traverse_types: unsafe { core::mem::zeroed() },
            no_traverse_type_packs: unsafe { core::mem::zeroed() },
        };

        base.substitution_txn_log_type_arena(TxnLog::empty(), arena);

        let mut this = ReplacerDeprecated {
            base,
            replacements,
            replacement_packs,
        };

        LUAU_ASSERT!(this.replacements.size() > 0 || this.replacement_packs.size() > 0 || true);

        this
    }
}
