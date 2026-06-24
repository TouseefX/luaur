use crate::functions::follow_type::follow_type_id;
use crate::functions::is_pending::is_pending;
use crate::functions::simplify_intersection_simplify::simplify_intersection;
use crate::functions::simplify_union::simplify_union;
use crate::records::simplify_result::SimplifyResult;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn and_type_function(
    instance: TypeId,
    type_params: alloc::vec::Vec<TypeId>,
    pack_params: alloc::vec::Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 2 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("and type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let lhs_ty = unsafe { follow_type_id(type_params[0]) };
    let rhs_ty = unsafe { follow_type_id(type_params[1]) };

    // t1 = and<lhs, t1> ~> lhs
    if unsafe { follow_type_id(rhs_ty) } == instance && lhs_ty != rhs_ty {
        return TypeFunctionReductionResult {
            result: Some(lhs_ty),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: alloc::vec::Vec::new(),
            blocked_packs: alloc::vec::Vec::new(),
            error: None,
            messages: alloc::vec::Vec::new(),
        };
    }
    // t1 = and<t1, rhs> ~> rhs
    if unsafe { follow_type_id(lhs_ty) } == instance && lhs_ty != rhs_ty {
        return TypeFunctionReductionResult {
            result: Some(rhs_ty),
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: alloc::vec::Vec::new(),
            blocked_packs: alloc::vec::Vec::new(),
            error: None,
            messages: alloc::vec::Vec::new(),
        };
    }

    // check to see if both operand types are resolved enough, and wait to reduce if not
    if is_pending(lhs_ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: alloc::vec::Vec::from([lhs_ty]),
            blocked_packs: alloc::vec::Vec::new(),
            error: None,
            messages: alloc::vec::Vec::new(),
        };
    } else if is_pending(rhs_ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: crate::enums::reduction::Reduction::MaybeOk,
            blocked_types: alloc::vec::Vec::from([rhs_ty]),
            blocked_packs: alloc::vec::Vec::new(),
            error: None,
            messages: alloc::vec::Vec::new(),
        };
    }

    // And evaluates to a boolean if the LHS is falsy, and the RHS type if LHS is truthy.
    let filtered_lhs = unsafe {
        simplify_intersection(
            ctx_ref.builtins.as_ptr(),
            ctx_ref.arena.as_ptr(),
            lhs_ty,
            ctx_ref.builtins.as_ref().falsyType,
        )
    };
    let overall_result = unsafe {
        simplify_union(
            ctx_ref.builtins.as_ptr(),
            ctx_ref.arena.as_ptr(),
            rhs_ty,
            filtered_lhs.result,
        )
    };

    let mut blocked_types: alloc::vec::Vec<TypeId> = alloc::vec::Vec::new();
    for ty in filtered_lhs.blocked_types.iter() {
        blocked_types.push(*ty);
    }
    for ty in overall_result.blocked_types.iter() {
        blocked_types.push(*ty);
    }

    TypeFunctionReductionResult {
        result: Some(overall_result.result),
        reduction_status: crate::enums::reduction::Reduction::MaybeOk,
        blocked_types,
        blocked_packs: alloc::vec::Vec::new(),
        error: None,
        messages: alloc::vec::Vec::new(),
    }
}
