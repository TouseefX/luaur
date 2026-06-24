use crate::functions::index_function_impl::index_function_impl;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn rawget_type_function(
    _instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let _ctx_ref = unsafe { &*ctx };
    if type_params.len() != 2 || !pack_params.is_empty() {
        LUAU_ASSERT!(false);
    }

    index_function_impl(type_params, pack_params, ctx, true)
}
