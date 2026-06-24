use crate::enums::reduction::Reduction;
use crate::functions::follow_type::follow_type_id;
use crate::functions::get_type_alt_j::get_type_id;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_instance_type::TypeFunctionInstanceType;
use crate::records::type_function_reduction_result::TypeFunctionReductionResult;
use crate::records::union_type::UnionType;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;

pub fn try_distribute_type_function_app<F>(
    mut f: F,
    instance: TypeId,
    type_params: &[TypeId],
    pack_params: &[TypePackId],
    ctx: *mut TypeFunctionContext,
) -> Option<TypeFunctionReductionResult>
where
    F: FnMut(
        TypeId,
        Vec<TypeId>,
        Vec<TypePackId>,
        *mut TypeFunctionContext,
    ) -> TypeFunctionReductionResult,
{
    let mut reduction_status = Reduction::MaybeOk;
    let mut blocked_types = Vec::new();
    let mut results = Vec::new();
    let mut cartesian_product_size = 1usize;

    let mut first_union_options: Option<Vec<TypeId>> = None;
    let mut union_index = 0usize;
    let mut arguments = type_params.to_vec();

    for (index, argument) in arguments.iter().copied().enumerate() {
        let ty = unsafe { follow_type_id(argument) };
        let union = unsafe { get_type_id::<UnionType>(ty).as_ref() };

        let Some(union) = union else {
            continue;
        };

        if first_union_options.is_none() {
            first_union_options = Some(union.options.clone());
            union_index = index;
        }

        cartesian_product_size = cartesian_product_size.saturating_mul(union.options.len());
        if (luaur_common::DFInt::LuauTypeFamilyApplicationCartesianProductLimit.get() as usize)
            <= cartesian_product_size
        {
            return Some(TypeFunctionReductionResult {
                result: None,
                reduction_status: Reduction::Erroneous,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            });
        }
    }

    let first_union_options = first_union_options?;

    for option in first_union_options {
        arguments[union_index] = option;

        let result = f(instance, arguments.clone(), pack_params.to_vec(), ctx);
        blocked_types.extend(result.blocked_types.iter().copied());

        if result.reduction_status != Reduction::MaybeOk {
            reduction_status = result.reduction_status;
        }

        if reduction_status != Reduction::MaybeOk || result.result.is_none() {
            break;
        }

        results.push(result.result.unwrap());
    }

    if reduction_status != Reduction::MaybeOk || !blocked_types.is_empty() {
        return Some(TypeFunctionReductionResult {
            result: None,
            reduction_status,
            blocked_types,
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        });
    }

    if !results.is_empty() {
        if results.len() == 1 {
            return Some(TypeFunctionReductionResult {
                result: Some(results[0]),
                reduction_status: Reduction::MaybeOk,
                blocked_types: Vec::new(),
                blocked_packs: Vec::new(),
                error: None,
                messages: Vec::new(),
            });
        }

        let result_ty = unsafe {
            let ctx_ref = &mut *ctx;
            let union_func = &ctx_ref.builtins.as_ref().typeFunctions.union_func;
            let ty = ctx_ref
                .arena
                .as_mut()
                .add_type(TypeFunctionInstanceType::new_with_args(union_func, results));
            ctx_ref.fresh_instances.push(ty);
            ty
        };

        return Some(TypeFunctionReductionResult {
            result: Some(result_ty),
            reduction_status: Reduction::MaybeOk,
            blocked_types: Vec::new(),
            blocked_packs: Vec::new(),
            error: None,
            messages: Vec::new(),
        });
    }

    None
}
