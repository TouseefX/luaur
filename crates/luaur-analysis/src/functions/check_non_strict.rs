//! C++ free function `Luau::checkNonStrict(...)`
//! (`Analysis/src/NonStrictTypeChecker.cpp:1287-1320`).

use crate::functions::copy_errors::copy_errors;
use crate::functions::freeze::freeze;
use crate::functions::unfreeze::unfreeze;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::data_flow_graph::DataFlowGraph;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module::Module;
use crate::records::non_strict_type_checker::NonStrictTypeChecker;
use crate::records::source_module::SourceModule;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::unifier_shared_state::UnifierSharedState;
use crate::type_aliases::type_error_data::TypeErrorData;

pub fn check_non_strict(
    builtin_types: *mut BuiltinTypes,
    type_function_runtime: *mut TypeFunctionRuntime,
    ice: *mut InternalErrorReporter,
    unifier_state: *mut UnifierSharedState,
    dfg: *const DataFlowGraph,
    limits: *mut TypeCheckLimits,
    source_module: &SourceModule,
    module: *mut Module,
) {
    let mut type_checker = unsafe {
        NonStrictTypeChecker::non_strict_type_checker(
            &mut (*module).internal_types,
            builtin_types,
            type_function_runtime,
            ice,
            unifier_state,
            dfg,
            limits,
            module,
        )
    };
    unsafe { type_checker.wire_self_pointers() };

    type_checker.visit_ast_stat_block(source_module.root);

    unsafe {
        let module = &mut *module;
        unfreeze(&mut module.interface_types);
        copy_errors(
            &mut module.errors,
            &mut module.interface_types,
            &*builtin_types,
        );

        module
            .errors
            .retain(|err| !matches!(err.data, TypeErrorData::UnknownRequire(_)));

        freeze(&mut module.interface_types);
    }
}
