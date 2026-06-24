//! `reduceTypeFunctions(TypePackId entrypoint, ...)` (TypeFunction.cpp:722-747).

use crate::functions::reduce_functions_internal::reduce_functions_internal;
use crate::records::function_graph_reduction_result::FunctionGraphReductionResult;
use crate::records::generic_type_visitor::GenericTypeVisitorTrait;
use crate::records::instance_collector::InstanceCollector;
use crate::records::type_function_context::TypeFunctionContext;
use crate::records::type_once_visitor::TypeOnceVisitor;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_type_pack_id_set::TypeOrTypePackIdSet;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::string::String;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::records::vec_deque::VecDeque;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

pub fn reduce_type_functions(
    entrypoint: TypePackId,
    location: Location,
    ctx: NonNull<TypeFunctionContext>,
    force: bool,
) -> FunctionGraphReductionResult {
    let mut collector = InstanceCollector {
        base: TypeOnceVisitor::new(String::from("InstanceCollector"), true),
        recorded_tys: DenseHashSet::new(TypeId::default()),
        tys: VecDeque::new(),
        recorded_tps: DenseHashSet::new(TypePackId::default()),
        tps: VecDeque::new(),
        should_guess: TypeOrTypePackIdSet::default(),
        type_function_instance_stack: Vec::new(),
        cyclic_instance: Vec::new(),
    };

    // C++ wraps this in `try { ... } catch (RecursionLimitException&) { return {}; }`.
    if let Err(payload) = catch_unwind(AssertUnwindSafe(|| {
        collector.traverse_type_pack_id(entrypoint)
    })) {
        if !is_recursion_limit_panic(&payload) {
            resume_unwind(payload);
        }

        return empty_reduction_result();
    }

    if collector.tys.empty() && collector.tps.empty() {
        return empty_reduction_result();
    }

    reduce_functions_internal(
        collector.tys,
        collector.tps,
        collector.should_guess,
        collector.cyclic_instance,
        location,
        ctx,
        force,
    )
}

fn empty_reduction_result() -> FunctionGraphReductionResult {
    FunctionGraphReductionResult {
        errors: alloc::vec::Vec::new(),
        messages: alloc::vec::Vec::new(),
        blocked_types: DenseHashSet::new(core::ptr::null()),
        blocked_packs: DenseHashSet::new(core::ptr::null()),
        reduced_types: DenseHashSet::new(core::ptr::null()),
        reduced_packs: DenseHashSet::new(core::ptr::null()),
        irreducible_types: DenseHashSet::new(core::ptr::null()),
    }
}

fn is_recursion_limit_panic(payload: &Box<dyn core::any::Any + Send>) -> bool {
    const PREFIX: &str = "Internal recursion counter limit exceeded";

    if let Some(message) = payload.downcast_ref::<&str>() {
        message.starts_with(PREFIX)
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.starts_with(PREFIX)
    } else {
        false
    }
}
