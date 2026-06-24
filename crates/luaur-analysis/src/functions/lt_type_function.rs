use crate::functions::comparison_type_function::comparison_type_function;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn lt_type_function(
    _instance: TypeId,
    _type_params: Vec<TypeId>,
    _pack_params: Vec<TypePackId>,
    _ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let _ctx_ref = unsafe { &*_ctx };
    if _type_params.len() != 2 || !_pack_params.is_empty() {
        LUAU_ASSERT!(false);
    }

    comparison_type_function(
        _instance,
        _type_params,
        _pack_params,
        _ctx,
        "__lt".to_string(),
    )
}
