use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::subtyping::Subtyping;
use crate::records::type_arena::TypeArena;
use crate::records::type_function_runtime::TypeFunctionRuntime;

impl Subtyping {
    pub fn subtyping_not_null_builtin_types_not_null_type_arena_not_null_normalizer_not_null_type_function_runtime_not_null_internal_error_reporter(
        &mut self,
        builtin_types: *mut BuiltinTypes,
        type_arena: *mut TypeArena,
        normalizer: *mut Normalizer,
        type_function_runtime: *mut TypeFunctionRuntime,
        ice_reporter: *mut InternalErrorReporter,
    ) {
        self.builtin_types = builtin_types;
        self.arena = type_arena;
        self.normalizer = normalizer;
        self.type_function_runtime = type_function_runtime;
        self.ice_reporter = ice_reporter;
    }
}
