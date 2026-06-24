use crate::enums::normalization_result::NormalizationResult;
use crate::functions::as_mutable_type::as_mutable_type_id;
use crate::functions::find_metatable_entry::find_metatable_entry;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_blocked_or_unsolved_type::is_blocked_or_unsolved_type;
use crate::functions::is_number::is_number;
use crate::functions::is_pending::is_pending;
use crate::functions::solve_function_call::solve_function_call;
use crate::functions::try_distribute_type_function_app::try_distribute_type_function_app;
use crate::methods::unifiable_bound_type_id_emplace_type_bound_type::unifiable_bound_type_id_emplace_type_bound_type;
use crate::records::free_type::FreeType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn comparison_type_function(
    instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
    metamethod: String,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 2 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string(
                "encountered a type function instance without the required argument structure",
            )
        };
        LUAU_ASSERT!(false);
    }

    let lhs_ty = unsafe { follow_type_id(type_params[0]) };
    let rhs_ty = unsafe { follow_type_id(type_params[1]) };

    if lhs_ty == instance || rhs_ty == instance {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).neverType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if is_blocked_or_unsolved_type(lhs_ty) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![lhs_ty],
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    } else if is_blocked_or_unsolved_type(rhs_ty) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![rhs_ty],
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    let can_submit_constraint = !ctx_ref.solver.is_null() && !ctx_ref.constraint.is_null();
    let lhs_free = unsafe { get_type_id::<FreeType>(lhs_ty).as_ref().is_some() };
    let rhs_free = unsafe { get_type_id::<FreeType>(rhs_ty).as_ref().is_some() };

    if can_submit_constraint {
        if lhs_free && is_number(rhs_ty) {
            unsafe {
                let mut number_type = (*ctx_ref.builtins.as_ptr()).numberType;
                unifiable_bound_type_id_emplace_type_bound_type(
                    &mut *as_mutable_type_id(lhs_ty),
                    &mut number_type,
                );
            }
        } else if rhs_free && is_number(lhs_ty) {
            unsafe {
                let mut number_type = (*ctx_ref.builtins.as_ptr()).numberType;
                unifiable_bound_type_id_emplace_type_bound_type(
                    &mut *as_mutable_type_id(rhs_ty),
                    &mut number_type,
                );
            }
        }
    }

    let lhs_ty = unsafe { follow_type_id(lhs_ty) };
    let rhs_ty = unsafe { follow_type_id(rhs_ty) };

    let norm_lhs_ty = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(lhs_ty) };
    let norm_rhs_ty = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(rhs_ty) };
    let lhs_inhabited = unsafe {
        (*ctx_ref.normalizer.as_ptr()).is_inhabited_normalized_type(norm_lhs_ty.as_ref())
    };
    let rhs_inhabited = unsafe {
        (*ctx_ref.normalizer.as_ptr()).is_inhabited_normalized_type(norm_rhs_ty.as_ref())
    };

    if lhs_inhabited == NormalizationResult::HitLimits
        || rhs_inhabited == NormalizationResult::HitLimits
    {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if norm_lhs_ty.should_suppress_errors() || norm_rhs_ty.should_suppress_errors() {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if lhs_inhabited == NormalizationResult::False || rhs_inhabited == NormalizationResult::False {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if norm_lhs_ty.is_subtype_of_string() && norm_rhs_ty.is_subtype_of_string() {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if norm_lhs_ty.is_exactly_number() && norm_rhs_ty.is_exactly_number() {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    if let Some(result) = try_distribute_type_function_app(
        |instance, type_params, pack_params, ctx| {
            comparison_type_function(instance, type_params, pack_params, ctx, metamethod.clone())
        },
        instance,
        &type_params,
        &pack_params,
        ctx,
    ) {
        return result;
    }

    let mut dummy: ErrorVec = Vec::new();
    let location = Location::new(
        luaur_ast::records::position::Position::default(),
        luaur_ast::records::position::Position::default(),
    );

    let mut mm_type = find_metatable_entry(
        ctx_ref.builtins.as_ptr(),
        &mut dummy,
        lhs_ty,
        &metamethod,
        location,
    );
    if mm_type.is_none() {
        mm_type = find_metatable_entry(
            ctx_ref.builtins.as_ptr(),
            &mut dummy,
            rhs_ty,
            &metamethod,
            location,
        );
    }

    if mm_type.is_none() {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    let mm_type = unsafe { follow_type_id(mm_type.unwrap()) };
    if is_pending(mm_type, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![mm_type],
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    let args_pack = unsafe {
        (*ctx_ref.arena.as_ptr()).add_type_pack_t(crate::records::type_pack::TypePack {
            head: vec![lhs_ty, rhs_ty],
            tail: None,
        })
    };

    let call_location = if !ctx_ref.constraint.is_null() {
        unsafe { (*ctx_ref.constraint).location }
    } else {
        location
    };
    if solve_function_call(ctx, call_location, mm_type, args_pack).is_none() {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        };
    }

    TypeFunctionReductionResult {
        result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
        reduction_status: crate::enums::reduction::Reduction::MaybeOk,
        blocked_types: Vec::new(),
        blocked_packs: Vec::new(),
        error: None,
        messages: Vec::new(),
    }
}
