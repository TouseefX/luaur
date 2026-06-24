use crate::records::builtin_types::BuiltinTypes;
use crate::records::def::Def;
use crate::records::non_strict_context::NonStrictContext;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::def_id_def::DefId;
use crate::type_aliases::type_id::TypeId;

impl NonStrictContext {
    pub fn find_def_id(&self, def: &DefId) -> Option<TypeId> {
        let d: *const Def = *def;
        self.find_def(d)
    }

    pub fn disjunction(
        builtin_types: *mut BuiltinTypes,
        arena: *mut TypeArena,
        left: &NonStrictContext,
        right: &NonStrictContext,
    ) -> NonStrictContext {
        crate::methods::non_strict_context_disjunction::non_strict_context_disjunction(
            builtin_types,
            arena,
            left,
            right,
        )
    }

    pub fn conjunction(
        builtin_types: *mut BuiltinTypes,
        arena: *mut TypeArena,
        left: &NonStrictContext,
        right: &NonStrictContext,
    ) -> NonStrictContext {
        crate::methods::non_strict_context_conjunction::non_strict_context_conjunction(
            builtin_types,
            arena,
            left,
            right,
        )
    }
}
