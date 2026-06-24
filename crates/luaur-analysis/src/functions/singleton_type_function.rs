use crate::enums::reduction::Reduction;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_pending::is_pending;
use crate::records::negation_type::NegationType;
use crate::records::singleton_type::SingletonType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::rtti::ast_node_is;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn singleton_type_function(
    _instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 1 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("singleton type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let ty = unsafe { follow_type_id(type_params[0]) };
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

    let mut followed = ty;
    if let Some(negation) = unsafe { get_type_id::<NegationType>(followed).as_ref() } {
        followed = unsafe { follow_type_id(negation.ty) };
    }

    if unsafe {
        get_type_id::<SingletonType>(followed).as_ref().is_some()
            || crate::functions::is_nil::is_nil(followed)
    } {
        return TypeFunctionReductionResult {
            result: Some(ty),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    TypeFunctionReductionResult {
        result: Some(unsafe { ctx_ref.builtins.as_ref().unknownType }),
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
    }
}
