use crate::functions::clone_clone_alt_b::with_clone_maps;
use crate::records::clone_state::CloneState;
use crate::records::type_arena::TypeArena;
use crate::records::type_cloner::TypeCloner;
use crate::records::type_fun::TypeFun;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use std::collections::HashMap;

pub fn clone(type_fun: &TypeFun, dest: &mut TypeArena, clone_state: &mut CloneState) -> TypeFun {
    let builtin_types = clone_state.builtin_types;
    with_clone_maps(
        &mut clone_state.seen_types,
        &mut clone_state.seen_type_packs,
        |tys, tps| {
            let mut cloner = TypeCloner {
                arena: dest as *mut TypeArena,
                builtin_types,
                queue: alloc::vec::Vec::new(),
                types: tys as *mut HashMap<TypeId, TypeId>,
                packs: tps as *mut HashMap<TypePackId, TypePackId>,
                force_ty: core::ptr::null(),
                force_tp: core::ptr::null(),
                steps: 0,
                replacement_for_null_scope: core::ptr::null_mut(),
                skip_lazy_type_clone: false,
            };

            let mut copy = type_fun.clone();

            for param in copy.type_params.iter_mut() {
                param.ty = cloner.clone_type_id(param.ty);

                if let Some(default_value) = param.defaultValue {
                    param.defaultValue = Some(cloner.clone_type_id(default_value));
                }
            }

            for param in copy.type_pack_params.iter_mut() {
                param.tp = cloner.clone_type_pack_id(param.tp);

                if let Some(default_value) = param.defaultValue {
                    param.defaultValue = Some(cloner.clone_type_pack_id(default_value));
                }
            }

            copy.r#type = cloner.clone_type_id(copy.r#type);

            copy
        },
    )
}
