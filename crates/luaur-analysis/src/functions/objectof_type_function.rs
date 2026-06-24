use crate::enums::reduction::Reduction;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::is_pending::is_pending;
use crate::records::extern_type::ExternType;
use crate::records::obj::Obj;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn objectof_type_function(
    _instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 1 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("objectof type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let target_ty = unsafe { follow_type_id(type_params[0]) };
    if is_pending(target_ty, ctx_ref.solver) {
        return TypeFunctionReductionResult {
            result: None,
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![target_ty],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if let Some(klass) = unsafe { get_type_id::<ExternType>(target_ty).as_ref() } {
        if let Some(ref relation) = klass.relation {
            if let Some(obj) = relation.get_if::<Obj>() {
                return TypeFunctionReductionResult {
                    result: Some(obj.ty),
                    reduction_status: Reduction::MaybeOk,
                    blocked_types: vec![],
                    blocked_packs: vec![],
                    error: None,
                    messages: vec![],
                };
            }
        }
    }

    TypeFunctionReductionResult {
        result: Some(unsafe { ctx_ref.builtins.as_ref().errorType }),
        reduction_status: Reduction::MaybeOk,
        blocked_types: vec![],
        blocked_packs: vec![],
        error: None,
        messages: vec![],
    }
}
