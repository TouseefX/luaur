use crate::enums::normalization_result::NormalizationResult;
use crate::enums::reduction::Reduction;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn eq_type_function(
    _instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 2 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("eq type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let lhs_ty = unsafe { crate::functions::follow_type::follow_type_id(type_params[0]) };
    let rhs_ty = unsafe { crate::functions::follow_type::follow_type_id(type_params[1]) };

    if crate::functions::is_pending::is_pending(lhs_ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![lhs_ty],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if crate::functions::is_pending::is_pending(rhs_ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![rhs_ty],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

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
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    let norm_lhs_ref = norm_lhs_ty.as_ref();
    let norm_rhs_ref = norm_rhs_ty.as_ref();

    if norm_lhs_ref.should_suppress_errors() || norm_rhs_ref.should_suppress_errors() {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if lhs_inhabited == NormalizationResult::False || rhs_inhabited == NormalizationResult::False {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    let mut dummy: ErrorVec = Vec::new();
    let mm_type = unsafe {
        crate::functions::find_metatable_entry::find_metatable_entry(
            ctx_ref.builtins.as_ptr(),
            &mut dummy,
            lhs_ty,
            "__eq",
            Location::new(
                luaur_ast::records::position::Position { line: 0, column: 0 },
                luaur_ast::records::position::Position { line: 0, column: 0 },
            ),
        )
    };

    let mm_type = if mm_type.is_none() {
        unsafe {
            crate::functions::find_metatable_entry::find_metatable_entry(
                ctx_ref.builtins.as_ptr(),
                &mut dummy,
                rhs_ty,
                "__eq",
                Location::new(
                    luaur_ast::records::position::Position { line: 0, column: 0 },
                    luaur_ast::records::position::Position { line: 0, column: 0 },
                ),
            )
        }
    } else {
        mm_type
    };

    let intersect_inhabited = unsafe {
        (*ctx_ref.normalizer.as_ptr()).is_intersection_inhabited_type_id_type_id(lhs_ty, rhs_ty)
    };

    let mm_type = if mm_type.is_none() {
        if intersect_inhabited == NormalizationResult::True {
            return TypeFunctionReductionResult {
                result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
                reduction_status: Reduction::MaybeOk,
                blocked_types: vec![],
                blocked_packs: vec![],
                error: None,
                messages: vec![],
            };
        }

        if intersect_inhabited == NormalizationResult::False {
            if norm_lhs_ref.is_subtype_of_string() && norm_rhs_ref.is_subtype_of_string() {
                return TypeFunctionReductionResult {
                    result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).falseType }),
                    reduction_status: Reduction::MaybeOk,
                    blocked_types: vec![],
                    blocked_packs: vec![],
                    error: None,
                    messages: vec![],
                };
            }

            if norm_lhs_ref.is_subtype_of_booleans() && norm_rhs_ref.is_subtype_of_booleans() {
                return TypeFunctionReductionResult {
                    result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).falseType }),
                    reduction_status: Reduction::MaybeOk,
                    blocked_types: vec![],
                    blocked_packs: vec![],
                    error: None,
                    messages: vec![],
                };
            }
        }

        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    } else {
        mm_type
    };

    let mm_type_followed =
        unsafe { crate::functions::follow_type::follow_type_id(mm_type.unwrap()) };
    if crate::functions::is_pending::is_pending(mm_type_followed, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![mm_type_followed],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    let args_pack = unsafe {
        (*ctx_ref.arena.as_ptr()).add_type_pack_t(crate::records::type_pack::TypePack {
            head: vec![lhs_ty, rhs_ty],
            tail: None,
        })
    };

    if crate::functions::solve_function_call::solve_function_call(
        ctx,
        if !ctx_ref.constraint.is_null() {
            unsafe { (*ctx_ref.constraint).location }
        } else {
            Location::new(
                luaur_ast::records::position::Position { line: 0, column: 0 },
                luaur_ast::records::position::Position { line: 0, column: 0 },
            )
        },
        mm_type.unwrap(),
        args_pack,
    )
    .is_none()
    {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    TypeFunctionReductionResult {
        result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).booleanType }),
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
    }
}
