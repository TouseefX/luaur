use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::overload_resolver::OverloadResolver;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use luaur_ast::records::location::Location;

impl OverloadResolver {
    pub fn new(
        builtin_types: *mut BuiltinTypes,
        arena: *mut TypeArena,
        normalizer: *mut Normalizer,
        type_function_runtime: *mut TypeFunctionRuntime,
        scope: *mut Scope,
        reporter: *mut InternalErrorReporter,
        limits: *mut TypeCheckLimits,
        call_location: Location,
    ) -> Self {
        Self {
            builtin_types,
            arena,
            normalizer,
            type_function_runtime,
            scope,
            ice: reporter,
            limits: unsafe { (*limits).clone() },
            subtyping: Subtyping::subtyping_owned(
                builtin_types,
                arena,
                normalizer,
                type_function_runtime,
                reporter,
            ),
            call_loc: call_location,
        }
    }
}
