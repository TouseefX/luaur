//! `reduceFunctionsInternal` (TypeFunction.cpp:657-693).

use crate::records::code_too_complex::CodeTooComplex;
use crate::records::function_graph_reduction_result::FunctionGraphReductionResult;
use crate::records::type_error::TypeError;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_function_reducer::TypeFunctionReducer;
use crate::records::type_reduction_reentrancy_guard::TypeReductionReentrancyGuard;
use crate::type_aliases::type_error_data::TypeErrorData;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_type_pack_id_set::TypeOrTypePackIdSet;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;
use luaur_common::records::vec_deque::VecDeque;

pub fn reduce_functions_internal(
    queued_tys: VecDeque<TypeId>,
    queued_tps: VecDeque<TypePackId>,
    should_guess: TypeOrTypePackIdSet,
    cyclics: Vec<TypeId>,
    location: Location,
    ctx: NonNull<TypeFunctionContext>,
    force: bool,
) -> FunctionGraphReductionResult {
    let mut reducer = TypeFunctionReducer::type_function_reducer(
        queued_tys,
        queued_tps,
        should_guess,
        cyclics,
        location,
        ctx,
        force,
    );
    let mut iteration_count: i32 = 0;

    // If we are reducing a type function while reducing a type function,
    // we're probably doing something clowny. One known place this can
    // occur is type function reduction => overload selection => subtyping
    // => back to type function reduction. At worst, if there's a reduction
    // that _doesn't_ loop forever and _needs_ reentrancy, we'll fail to
    // handle that and potentially emit an error when we didn't need to.
    let shared_state = unsafe { (*(*ctx.as_ptr()).normalizer.as_ptr()).shared_state };
    if unsafe { !shared_state.is_null() && (*shared_state).reentrant_type_reduction } {
        return FunctionGraphReductionResult {
            errors: alloc::vec::Vec::new(),
            messages: alloc::vec::Vec::new(),
            blocked_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
            blocked_packs: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
            reduced_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
            reduced_packs: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
            irreducible_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
        };
    }

    // TypeReductionReentrancyGuard _{ctx->normalizer->sharedState};
    // RAII: sets reentrant_type_reduction = true now, resets to false on scope exit.
    let _guard =
        TypeReductionReentrancyGuard::type_reduction_reentrancy_guard_not_null_unifier_shared_state(
            shared_state,
        );

    let max_steps = luaur_common::DFInt::LuauTypeFamilyGraphReductionMaximumSteps.get();

    while !reducer.done() {
        reducer.step();

        iteration_count += 1;
        if iteration_count > max_steps {
            reducer
                .result
                .errors
                .push(TypeError::type_error_location_type_error_data(
                    location,
                    TypeErrorData::CodeTooComplex(CodeTooComplex::default()),
                ));
            break;
        }
    }

    // The Rust `TypeReductionReentrancyGuard` has no `Drop` impl yet, so mirror
    // the C++ destructor (`sharedState->reentrantTypeReduction = false`) here at
    // scope exit to preserve the RAII semantics faithfully.
    unsafe {
        if !shared_state.is_null() {
            (*shared_state).reentrant_type_reduction = false;
        }
    }
    drop(_guard);

    core::mem::replace(
        &mut reducer.result,
        FunctionGraphReductionResult {
            errors: alloc::vec::Vec::new(),
            messages: alloc::vec::Vec::new(),
            blocked_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
            blocked_packs: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
            reduced_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
            reduced_packs: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
            irreducible_types: luaur_common::records::dense_hash_set::DenseHashSet::new(
                core::ptr::null(),
            ),
        },
    )
}
