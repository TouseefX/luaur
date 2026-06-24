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
    pub fn overload_resolver_not_null_builtin_types_not_null_type_arena_not_null_normalizer_not_null_type_function_runtime_not_null_scope_not_null_internal_error_reporter_not_null_type_check_limits_location(
        &mut self,
        builtin_types: *mut BuiltinTypes,
        arena: *mut TypeArena,
        normalizer: *mut Normalizer,
        type_function_runtime: *mut TypeFunctionRuntime,
        scope: *mut Scope,
        reporter: *mut InternalErrorReporter,
        limits: *mut TypeCheckLimits,
        call_location: Location,
    ) {
        self.builtin_types = builtin_types;
        self.arena = arena;
        self.normalizer = normalizer;
        self.type_function_runtime = type_function_runtime;
        self.scope = scope;
        self.ice = reporter;
        self.limits = unsafe { core::ptr::read(limits as *const TypeCheckLimits) };
        self.call_loc = call_location;

        // subtyping({builtinTypes, arena, normalizer, typeFunctionRuntime, ice})
        let subtyping = Subtyping::subtyping_owned(
            builtin_types,
            arena,
            normalizer,
            type_function_runtime,
            reporter,
        );
        self.subtyping = subtyping;
    }
}
