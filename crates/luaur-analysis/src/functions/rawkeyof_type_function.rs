use crate::functions::keyof_function_impl::keyof_function_impl;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn rawkeyof_type_function(
    _instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    if type_params.len() != 1 || !pack_params.is_empty() {
        LUAU_ASSERT!(false);
    }

    keyof_function_impl(type_params, pack_params, ctx, true)
}
