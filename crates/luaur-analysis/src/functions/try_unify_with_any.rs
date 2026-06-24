use crate::functions::get_mutable_type::get_mutable_type_id;
use crate::functions::queue_type_pack::queue_type_pack;
use crate::records::extern_type::ExternType;
use crate::records::free_type::FreeType;
use crate::records::function_type::FunctionType;
use crate::records::intersection_type::IntersectionType;
use crate::records::metatable_type::MetatableType;
use crate::records::r#type::Type;
use crate::records::table_type::TableType;
use crate::records::type_arena::TypeArena;
use crate::records::unifier::Unifier;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use crate::type_aliases::type_variant::TypeVariant;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn try_unify_with_any(
    queue: &mut Vec<TypeId>,
    state: &mut Unifier,
    seen: &mut DenseHashSet<TypeId>,
    seen_type_packs: &mut DenseHashSet<TypePackId>,
    type_arena: *const TypeArena,
    any_type: TypeId,
    any_type_pack: TypePackId,
) {
    while !queue.is_empty() {
        let ty = state.log.follow_type_id(*queue.last().unwrap());
        queue.pop();

        if unsafe { (*ty).owning_arena } as *const TypeArena != type_arena {
            continue;
        }

        if seen.find(&ty).is_some() {
            continue;
        }

        seen.insert(ty);

        if !unsafe { get_mutable_type_id::<FreeType>(ty) }.is_null() {
            state
                .log
                .replace_type_id_t(ty, Type::new(TypeVariant::Bound(any_type)));
        } else if let Some(fun) = unsafe { get_mutable_type_id::<FunctionType>(ty).as_mut() } {
            queue_type_pack(queue, seen_type_packs, state, fun.arg_types, any_type_pack);
            queue_type_pack(queue, seen_type_packs, state, fun.ret_types, any_type_pack);
        } else if let Some(table) = unsafe { get_mutable_type_id::<TableType>(ty).as_mut() } {
            for (_name, prop) in table.props.iter() {
                if let Some(prop_ty) = prop.read_ty.or(prop.write_ty) {
                    queue.push(prop_ty);
                }
            }

            if let Some(indexer) = &table.indexer {
                queue.push(indexer.index_type);
                queue.push(indexer.index_result_type);
            }
        } else if let Some(mt) = unsafe { get_mutable_type_id::<MetatableType>(ty).as_mut() } {
            queue.push(mt.table);
            queue.push(mt.metatable);
        } else if !unsafe { get_mutable_type_id::<ExternType>(ty) }.is_null() {
        } else if let Some(union_) = unsafe { get_mutable_type_id::<UnionType>(ty).as_mut() } {
            queue.extend(union_.options.iter().copied());
        } else if let Some(intersection) =
            unsafe { get_mutable_type_id::<IntersectionType>(ty).as_mut() }
        {
            queue.extend(intersection.parts.iter().copied());
        } else {
        }
    }
}
