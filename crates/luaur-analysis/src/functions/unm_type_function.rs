use crate::enums::reduction::Reduction;
use crate::functions::first::first;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_pending::is_pending;
use crate::functions::solve_function_call::solve_function_call;
use crate::functions::try_distribute_type_function_app::try_distribute_type_function_app;
use crate::records::never_type::NeverType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec;
use alloc::vec::Vec;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

fn empty_location() -> Location {
    Location::new(
        Position { line: 0, column: 0 },
        Position { line: 0, column: 0 },
    )
}

pub fn unm_type_function(
    instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 1 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("unm type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let mut operand_ty = unsafe { follow_type_id(type_params[0]) };

    if operand_ty == instance {
        return TypeFunctionReductionResult {
            result: Some(unsafe { ctx_ref.builtins.as_ref().neverType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    // check to see if the operand type is resolved enough, and wait to reduce if not
    if is_pending(operand_ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![operand_ty],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    operand_ty = unsafe { follow_type_id(operand_ty) };

    let norm_ty = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(operand_ty) };

    // NOTE: in C++ a failed normalization yields a null shared_ptr; the Rust
    // normalizer returns an owning handle, so we treat `should_suppress_errors`
    // and the subsequent checks directly, mirroring the inhabited/suppress flow.

    // if the operand is error suppressing, we can just go ahead and reduce.
    if unsafe { (*norm_ty).should_suppress_errors() } {
        return TypeFunctionReductionResult {
            result: Some(operand_ty),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    // if we have a `never`, we can never observe that the operation didn't work.
    if !unsafe { get_type_id::<NeverType>(operand_ty) }.is_null() {
        return TypeFunctionReductionResult {
            result: Some(unsafe { ctx_ref.builtins.as_ref().neverType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    // If the type is exactly `number`, we can reduce now.
    if unsafe { (*norm_ty).is_exactly_number() } {
        return TypeFunctionReductionResult {
            result: Some(unsafe { ctx_ref.builtins.as_ref().numberType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if let Some(result) = try_distribute_type_function_app(
        unm_type_function,
        instance,
        &type_params,
        &pack_params,
        ctx,
    ) {
        return result;
    }

    // findMetatableEntry demands the ability to emit errors, so we must give it
    // the necessary state to do that, even if we intend to just eat the errors.
    let mut dummy: ErrorVec = vec![];

    let mm_type = unsafe {
        crate::functions::find_metatable_entry::find_metatable_entry(
            ctx_ref.builtins.as_ptr(),
            &mut dummy,
            operand_ty,
            "__unm",
            empty_location(),
        )
    };
    if mm_type.is_none() {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
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
            reduction_status: Reduction::MaybeOk,
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

    let location = if !ctx_ref.constraint.is_null() {
        unsafe { (*ctx_ref.constraint).location }
    } else {
        empty_location()
    };

    let result = solve_function_call(ctx, location, mm_type_followed, args_pack);
    let result = match result {
        Some(r) => r,
        None => {
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: vec![],
                blocked_packs: vec![],
                error: None,
                messages: vec![],
            };
        }
    };

    if let Some(ret) = first(result, true) {
        TypeFunctionReductionResult {
            result: Some(ret),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        }
    } else {
        TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        }
    }
}
