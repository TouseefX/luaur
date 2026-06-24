//! Source: `Analysis/src/Clone.cpp:677-710`
//! `TypeFun cloneIncremental(const TypeFun& typeFun, TypeArena& dest, CloneState& cloneState, Scope* freshScopeForFreeTypes)`.

use crate::functions::clone_clone_alt_b::with_clone_maps;
use crate::records::clone_state::CloneState;
use crate::records::fragment_autocomplete_type_cloner::FragmentAutocompleteTypeCloner;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use std::collections::HashMap;

pub fn clone_incremental(
    type_fun: &TypeFun,
    dest: &mut TypeArena,
    clone_state: &mut CloneState,
    fresh_scope_for_free_types: *mut Scope,
) -> TypeFun {
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

            let mut copy = type_fun.clone();

            for param in copy.type_params.iter_mut() {
                param.ty = cloner.base.clone_type_id(param.ty);

                if let Some(default_value) = param.defaultValue {
                    param.defaultValue = Some(cloner.base.clone_type_id(default_value));
                }
            }

            for param in copy.type_pack_params.iter_mut() {
                param.tp = cloner.base.clone_type_pack_id(param.tp);

                if let Some(default_value) = param.defaultValue {
                    param.defaultValue = Some(cloner.base.clone_type_pack_id(default_value));
                }
            }

            copy.r#type = cloner.base.clone_type_id(copy.r#type);

            copy
        },
    )
}
