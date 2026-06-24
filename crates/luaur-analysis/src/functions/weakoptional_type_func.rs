use crate::enums::normalization_result::NormalizationResult;
use crate::enums::reduction::Reduction;
use crate::functions::follow_type::follow_type_id;
use crate::functions::is_pending::is_pending;
use crate::records::never_type::NeverType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn weakoptional_type_func(
    instance: TypeId,
    type_params: alloc::vec::Vec<TypeId>,
    pack_params: alloc::vec::Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 1 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("weakoptional type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let target_ty = unsafe { follow_type_id(type_params[0]) };

    if is_pending(target_ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: alloc::vec![target_ty],
            blocked_packs: alloc::vec![],
            error: None,
            messages: alloc::vec![],
        };
    }

    if unsafe {
        crate::functions::get_type_alt_j::get_type_id::<NeverType>(instance)
            .as_ref()
            .is_some()
    } {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).nilType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: alloc::vec![],
            blocked_packs: alloc::vec![],
            error: None,
            messages: alloc::vec![],
        };
    }

    let target_norm = unsafe { (*ctx_ref.normalizer.as_ptr()).normalize(target_ty) };

    let result = unsafe {
        (*ctx_ref.normalizer.as_ptr()).is_inhabited_normalized_type(target_norm.as_ref())
    };
    if result == NormalizationResult::False {
        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).nilType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: alloc::vec![],
            blocked_packs: alloc::vec![],
            error: None,
            messages: alloc::vec![],
        };
    }

    TypeFunctionReductionResult {
        result: Some(target_ty),
        reduction_status: Reduction::MaybeOk,
        blocked_types: alloc::vec![],
        blocked_packs: alloc::vec![],
        error: None,
        messages: alloc::vec![],
    }
}
