use crate::functions::follow_type::follow_type_id;
use crate::functions::follow_type_utils::follow_optional_ty;
use crate::functions::generalize_type::generalize_type;
use crate::functions::generalize_type_pack::generalize_type_pack;
use crate::functions::seal_table::seal_table;
use crate::methods::free_type_searcher_visit_generalization_alt_c::searcher_traverse_type_id;
use crate::methods::type_cacher_visit_generalization_alt_i::cacher_traverse_type_id;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::free_type_searcher::FreeTypeSearcher;
use crate::records::function_type::FunctionType;
use crate::records::generalization_params::GeneralizationParams;
use crate::records::generalization_result::GeneralizationResult;
use crate::records::scope::Scope;
use crate::records::type_arena::TypeArena;
use crate::records::type_cacher::TypeCacher;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::records::dense_hash_set::DenseHashSet;

pub fn generalize(
    arena: *mut TypeArena,
    builtin_types: *mut BuiltinTypes,
    scope: *mut Scope,
    cached_types: *mut DenseHashSet<TypeId>,
    ty: TypeId,
    generalization_target: Option<TypeId>,
) -> Option<TypeId> {
    let ty = unsafe { follow_type_id(ty) };

    let ty_ptr = unsafe { &*ty };
    if ty_ptr.owning_arena != arena || ty_ptr.persistent {
        return Some(ty);
    }

    let mut fts = FreeTypeSearcher::free_type_searcher(scope, cached_types);
    searcher_traverse_type_id(&mut fts, ty);

    let function_ty =
        unsafe { crate::functions::get_mutable_type::get_mutable_type_id::<FunctionType>(ty) };
    let mut push_generic = |t: TypeId| {
        if !function_ty.is_null() {
            unsafe { (*function_ty).generics.push(t) };
        }
    };

    let mut push_generic_pack = |tp: TypePackId| {
        if !function_ty.is_null() {
            unsafe { (*function_ty).generic_packs.push(tp) };
        }
    };

    for (free_ty, params) in &fts.types {
        if !generalization_target.is_some() || *free_ty == generalization_target.unwrap() {
            let res: GeneralizationResult =
                generalize_type(arena, builtin_types, scope, *free_ty, &params);

            if res.resource_limits_exceeded {
                return None;
            }

            if res.was_replaced_by_generic && res.result.is_some() {
                push_generic(res.result.unwrap());
            }
        }
    }

    let unsealed_tables_iter = fts.unsealed_tables.iter();
    for unsealed_table_ty in unsealed_tables_iter {
        if !generalization_target.is_some() || *unsealed_table_ty == generalization_target.unwrap()
        {
            seal_table(scope, *unsealed_table_ty);
        }
    }

    for (free_pack_id, params) in &fts.type_packs {
        let free_pack =
            unsafe { crate::functions::follow_type_pack::follow_type_pack_id(*free_pack_id) };
        if generalization_target.is_none() {
            let generalized_tp: GeneralizationResult =
                generalize_type_pack(arena, builtin_types, scope, free_pack, &params);

            if generalized_tp.resource_limits_exceeded {
                return None;
            }

            if generalized_tp.was_replaced_by_generic && generalized_tp.result.is_some() {
                push_generic_pack(free_pack);
            }
        }
    }

    let mut cacher = TypeCacher::type_cacher(cached_types);
    cacher_traverse_type_id(&mut cacher, ty);

    Some(ty)
}
