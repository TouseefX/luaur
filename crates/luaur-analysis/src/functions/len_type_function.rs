use crate::enums::normalization_result::NormalizationResult;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_pending::is_pending;
use crate::functions::solve_function_call::solve_function_call;
use crate::functions::try_distribute_type_function_app::try_distribute_type_function_app;
use crate::records::metatable_type::MetatableType;
use crate::records::table_type::TableType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn len_type_function(
    instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 1 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("len type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let operand_ty = unsafe { follow_type_id(type_params[0]) };

    if operand_ty == instance {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ref()).neverType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if is_pending(operand_ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![operand_ty],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    let norm_ty = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(operand_ty) };
    let inhabited =
        unsafe { (*ctx_ref.normalizer.as_ptr()).is_inhabited_normalized_type(norm_ty.as_ref()) };

    if inhabited == NormalizationResult::HitLimits {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if unsafe { (*norm_ty).should_suppress_errors() } {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ref()).numberType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if inhabited == NormalizationResult::False || unsafe { (*norm_ty).is_subtype_of_string() } {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ref()).numberType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    let normalized_operand = unsafe {
        follow_type_id((*ctx_ref.normalizer.as_ptr()).type_from_normal(norm_ty.as_ref()))
    };

    if unsafe { (*norm_ty).has_top_table() }
        || !unsafe { get_type_id::<TableType>(normalized_operand) }.is_null()
    {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ref()).numberType }),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if let Some(result) = try_distribute_type_function_app(
        len_type_function,
        instance,
        &type_params,
        &pack_params,
        ctx,
    ) {
        return result;
    }

    let mut dummy: ErrorVec = vec![];
    let mm_type = unsafe {
        crate::functions::find_metatable_entry::find_metatable_entry(
            ctx_ref.builtins.as_ptr(),
            &mut dummy,
            operand_ty,
            "__len",
            Location::new(
                luaur_ast::records::position::Position { line: 0, column: 0 },
                luaur_ast::records::position::Position { line: 0, column: 0 },
            ),
        )
    };

    if mm_type.is_none() {
        if !unsafe { get_type_id::<MetatableType>(normalized_operand) }.is_null() {
            return TypeFunctionReductionResult {
                result: Some(unsafe { (*ctx_ref.builtins.as_ref()).numberType }),
                reduction_status: crate::enums::reduction::Reduction::MaybeOk,
                blocked_types: vec![],
                blocked_packs: vec![],
                error: None,
                messages: vec![],
            };
        }

        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::Erroneous,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    let mm_type_followed = unsafe { follow_type_id(mm_type.unwrap()) };
    if is_pending(mm_type_followed, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: vec![mm_type_followed],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    let args_pack = unsafe {
        (*ctx_ref.arena.as_ptr()).add_type_pack_t(crate::records::type_pack::TypePack {
            head: vec![operand_ty],
            tail: None,
        })
    };

    if solve_function_call(
        ctx,
        if !ctx_ref.constraint.is_null() {
            unsafe { (*ctx_ref.constraint).location }
        } else {
            Location::new(
                luaur_ast::records::position::Position { line: 0, column: 0 },
                luaur_ast::records::position::Position { line: 0, column: 0 },
            )
        },
        mm_type_followed,
        args_pack,
    )
    .is_none()
    {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::Erroneous,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    TypeFunctionReductionResult {
        result: Some(unsafe { (*ctx_ref.builtins.as_ref()).numberType }),
        reduction_status: crate::enums::reduction::Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
    }
}
