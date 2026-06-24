use crate::functions::numeric_binop_type_function::numeric_binop_type_function;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn mod_type_function(
    instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let _ctx_ref = unsafe { &*ctx };
    if type_params.len() != 2 || !pack_params.is_empty() {
        LUAU_ASSERT!(false);
    }

    numeric_binop_type_function(instance, type_params, pack_params, ctx, "__mod".to_string())
}
