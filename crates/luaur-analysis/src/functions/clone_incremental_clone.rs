//! Source: `Analysis/src/Clone.cpp:643-658`
//! `TypePackId cloneIncremental(TypePackId tp, TypeArena& dest, CloneState& cloneState, Scope* freshScopeForFreeTypes)`.

use crate::functions::clone_clone_alt_b::with_clone_maps;
use crate::records::clone_state::CloneState;
use crate::records::fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use std::collections::HashMap;

pub fn clone_incremental(
    tp: TypePackId,
    dest: &mut TypeArena,
    clone_state: &mut CloneState,
    fresh_scope_for_free_types: *mut Scope,
) -> TypePackId {
    if unsafe { (*tp).persistent } {
        return tp;
    }

    let builtin_types = clone_state.builtin_types;
    with_clone_maps(
        &mut clone_state.seen_types,
        &mut clone_state.seen_type_packs,
        |tys, tps| {
            let mut cloner = FragmentAutocompleteTypeCloner::fragment_autocomplete_type_cloner(
                dest as *mut TypeArena,
                builtin_types,
                tys as *mut HashMap<TypeId, TypeId>,
                tps as *mut HashMap<TypePackId, TypePackId>,
                core::ptr::null(),
                core::ptr::null(),
                fresh_scope_for_free_types,
            );
            cloner.base.clone_type_pack_id(tp)
        },
    )
}
