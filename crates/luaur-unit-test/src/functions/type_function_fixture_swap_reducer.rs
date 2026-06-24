use alloc::vec::Vec;
use luaur_analysis::enums::reduction::Reduction;
use luaur_analysis::functions::follow_type::follow_type_id;
use luaur_analysis::functions::is_number::is_number;
use luaur_analysis::functions::is_pending::is_pending;
use luaur_analysis::functions::is_string::is_string;
use luaur_analysis::records::type_function_context::TypeFunctionContext;
use luaur_analysis::records::type_function_reduction_result::TypeFunctionReductionResult;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn type_function_fixture_swap_reducer(
    _instance: TypeId,
    tys: Vec<TypeId>,
    tps: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    LUAU_ASSERT!(tys.len() == 1);
    LUAU_ASSERT!(tps.is_empty());

    let ctx_ref = unsafe { &*ctx };
    let param = unsafe { follow_type_id(tys[0]) };

    if is_string(param) {
        TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).numberType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        }
    } else if is_number(param) {
        TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.builtins.as_ptr()).stringType }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        }
    } else if is_pending(param, ctx_ref.solver) {
        TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: alloc::vec![param],
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        }
    } else {
        TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::Erroneous,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        }
    }
}
