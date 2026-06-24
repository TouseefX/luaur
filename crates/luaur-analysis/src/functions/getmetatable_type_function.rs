use crate::enums::reduction::Reduction;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::functions::getmetatable_helper::getmetatable_helper;
use crate::functions::is_pending::is_pending;
use crate::records::intersection_type::IntersectionType;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::records::union_type::UnionType;
use crate::records::unknown_type::UnknownType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::location::Location;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

pub fn getmetatable_type_function(
    _instance: TypeId,
    type_params: Vec<TypeId>,
    pack_params: Vec<TypePackId>,
    ctx: *mut TypeFunctionContext,
) -> TypeFunctionReductionResult {
    let ctx_ref = unsafe { &*ctx };
    if type_params.len() != 1 || !pack_params.is_empty() {
        unsafe {
            (*ctx_ref.ice.as_ptr()).ice_string("getmetatable type function: encountered a type function instance without the required argument structure")
        };
        LUAU_ASSERT!(false);
    }

    let location = if !ctx_ref.constraint.is_null() {
        unsafe { (*ctx_ref.constraint).location }
    } else {
        Location::new(Default::default(), Default::default())
    };

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

    if let Some(ut) = unsafe { get_type_id::<UnionType>(target_ty).as_ref() } {
        let mut options: Vec<TypeId> = Vec::with_capacity(ut.options.len());
        for option in &ut.options {
            let result = getmetatable_helper(*option, &location, ctx);
            if result.result.is_none() {
                return result;
            }
            options.push(*result.result.as_ref().unwrap());
        }

        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.arena.as_ptr()).add_type(UnionType { options }) }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    if let Some(it) = unsafe { get_type_id::<IntersectionType>(target_ty).as_ref() } {
        let mut parts: Vec<TypeId> = Vec::with_capacity(it.parts.len());
        let mut errored_with_unknown = false;

        for part in &it.parts {
            let result = getmetatable_helper(*part, &location, ctx);
            if result.result.is_none() {
                if unsafe {
                    get_type_id::<UnknownType>(follow_type_id(*part))
                        .as_ref()
                        .is_some()
                } {
                    errored_with_unknown = true;
                    continue;
                } else {
                    return result;
                }
            }
            parts.push(*result.result.as_ref().unwrap());
        }

        if errored_with_unknown && parts.is_empty() {
            return TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: vec![],
                blocked_packs: vec![],
                error: None,
                messages: vec![],
            };
        }

        if parts.len() == 1 {
            return TypeFunctionReductionResult {
                result: Some(parts[0]),
                reduction_status: Reduction::MaybeOk,
                blocked_types: vec![],
                blocked_packs: vec![],
                error: None,
                messages: vec![],
            };
        }

        return TypeFunctionReductionResult {
            result: Some(unsafe { (*ctx_ref.arena.as_ptr()).add_type(IntersectionType { parts }) }),
            reduction_status: Reduction::MaybeOk,
            blocked_types: vec![],
            blocked_packs: vec![],
            error: None,
            messages: vec![],
        };
    }

    getmetatable_helper(target_ty, &location, ctx)
}
