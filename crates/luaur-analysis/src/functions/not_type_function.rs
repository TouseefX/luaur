use crate::enums::reduction::Reduction;
use crate::functions::follow_type::follow_type_id;
use crate::functions::is_pending::is_pending;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn not_type_function(
    instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 1 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("not type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let ty = unsafe { follow_type_id(type_params[0]) };
    if ty == instance {
        return TypeFunctionReductionResult {
            result: Some(unsafe { ctx_ref.builtins.as_ref().neverType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if is_pending(ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![ty],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    TypeFunctionReductionResult {
        result: Some(unsafe { ctx_ref.builtins.as_ref().booleanType }),
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
    }
}
