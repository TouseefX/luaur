use crate::enums::reduction::Reduction;
use crate::records::metatable_type::MetatableType;
use crate::records::normalized_type::NormalizedType;
use crate::records::simplify_result::SimplifyResult;
use crate::records::table_type::TableType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn setmetatable_type_function(
    _instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    if type_params.len() != 2 || !pack_params.is_empty() {
        unsafe {
            LUAU_ASSERT!(false);
        }
    }

    let location = unsafe {
        let ctx_ref = &*ctx;
        if !ctx_ref.constraint.is_null() {
            (*ctx_ref.constraint).location
        } else {
            Location::new(
                luaur_ast::records::position::Position::default(),
                luaur_ast::records::position::Position::default(),
            )
        }
    };

    let target_ty = unsafe { crate::functions::follow_type::follow_type_id(type_params[0]) };
    let metatable_ty = unsafe { crate::functions::follow_type::follow_type_id(type_params[1]) };

    if unsafe { crate::functions::is_pending::is_pending(target_ty, (*ctx).solver) } {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![target_ty],
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    let target_norm = unsafe { (*(*ctx).normalizer.as_ptr()).normalize(target_ty) };

    let target_norm_ref = target_norm.as_ref();
    if !target_norm_ref.has_tables() {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if target_norm_ref.has_tops()
        || target_norm_ref.has_booleans()
        || target_norm_ref.has_errors()
        || target_norm_ref.has_nils()
        || target_norm_ref.has_numbers()
        || target_norm_ref.has_strings()
        || target_norm_ref.has_threads()
        || target_norm_ref.has_buffers()
        || target_norm_ref.has_functions()
        || target_norm_ref.has_tyvars()
        || target_norm_ref.has_extern_types()
    {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if unsafe { crate::functions::is_pending::is_pending(metatable_ty, (*ctx).solver) } {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![metatable_ty],
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    let metatable_is_table_or_metatable = unsafe {
        !crate::functions::get_type_alt_j::get_type_id::<TableType>(metatable_ty).is_null()
            || !crate::functions::get_type_alt_j::get_type_id::<MetatableType>(metatable_ty)
                .is_null()
    };

    if !metatable_is_table_or_metatable {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if target_norm_ref.tables.size() == 1 {
        let table = target_norm_ref.tables.front();

        let mut dummy: ErrorVec = Vec::new();

        let metatable_metamethod = unsafe {
            crate::functions::find_metatable_entry::find_metatable_entry(
                (*ctx).builtins.as_ptr(),
                &mut dummy,
                table,
                "__metatable",
                location.clone(),
            )
        };

        if metatable_metamethod.is_some() {
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            };
        }

        let with_metatable = unsafe {
            (*(*ctx).arena.as_ptr()).add_type(MetatableType {
                table,
                metatable: metatable_ty,
                syntheticName: None,
            })
        };

        return TypeFunctionReductionResult {
            result: Some(with_metatable),
            reduction_status: Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    let mut result = unsafe { (*(*ctx).builtins.as_ptr()).neverType };

    for component_ty in target_norm_ref.tables.order.iter().copied() {
        let mut dummy: ErrorVec = Vec::new();

        let metatable_metamethod = unsafe {
            crate::functions::find_metatable_entry::find_metatable_entry(
                (*ctx).builtins.as_ptr(),
                &mut dummy,
                component_ty,
                "__metatable",
                location.clone(),
            )
        };

        if metatable_metamethod.is_some() {
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            };
        }

        let with_metatable = unsafe {
            (*(*ctx).arena.as_ptr()).add_type(MetatableType {
                table: component_ty,
                metatable: metatable_ty,
                syntheticName: None,
            })
        };

        let simplified = unsafe {
            crate::functions::simplify_union::simplify_union(
                (*ctx).builtins.as_ptr(),
                (*ctx).arena.as_ptr(),
                result,
                with_metatable,
            )
        };

        if !simplified.blocked_types.empty() {
            let mut blocked_types = Vec::new();
            for ty in simplified.blocked_types.iter() {
                blocked_types.push(*ty);
            }
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::MaybeOk,
                blocked_types,
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            };
        }

        result = simplified.result;
    }

    TypeFunctionReductionResult {
        result: Some(result),
        reduction_status: Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
    }
}
