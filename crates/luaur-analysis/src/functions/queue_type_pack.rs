use crate::functions::get_mutable_type_pack::get_mutable_type_pack_id;
use crate::records::free_type_pack::FreeTypePack;
use crate::records::type_pack::TypePack;
use crate::records::type_pack_var::TypePackVar;
use crate::records::unifier::Unifier;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_pack_variant::TypePackVariant;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn queue_type_pack(
    queue: &mut Vec<TypeId>,
    seen_type_packs: &mut DenseHashSet<TypePackId>,
    state: &mut Unifier,
    mut a: TypePackId,
    any_type_pack: TypePackId,
) {
    loop {
        a = state.log.follow_type_pack_id(a);

        if seen_type_packs.find(&a).is_some() {
            break;
        }
        seen_type_packs.insert(a);

        if !unsafe { get_mutable_type_pack_id::<FreeTypePack>(a) }.is_null() {
            state.log.replace_type_pack_id_type_pack_var(
                a,
                TypePackVar {
                    ty: TypePackVariant::Bound(any_type_pack),
                    persistent: false,
                    owningArena: core::ptr::null_mut(),
                },
            );
        } else {
            let tp = unsafe { get_mutable_type_pack_id::<TypePack>(a) };
            if !tp.is_null() {
                let head = unsafe { &(*tp).head };
                queue.extend(head.iter().copied());

                if let Some(tail) = unsafe { (*tp).tail } {
                    a = tail;
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}
