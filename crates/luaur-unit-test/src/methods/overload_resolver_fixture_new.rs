use crate::records::fixture::Fixture;
use crate::records::overload_resolver_fixture::OverloadResolverFixture;
use alloc::boxed::Box;
use alloc::sync::Arc;
use luaur_analysis::enums::solver_mode::SolverMode;
use luaur_analysis::records::builtin_types::BuiltinTypes;
use luaur_analysis::records::function_type::FunctionType;
use luaur_analysis::records::internal_error_reporter::InternalErrorReporter;
use luaur_analysis::records::intersection_type::IntersectionType;
use luaur_analysis::records::normalizer::Normalizer;
use luaur_analysis::records::overload_resolver::OverloadResolver;
use luaur_analysis::records::r#type::Type;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_analysis::records::type_function_runtime::TypeFunctionRuntime;
use luaur_analysis::records::unifier_shared_state::UnifierSharedState;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_set::DenseHashSet;
use luaur_common::FFlag;

impl OverloadResolverFixture {
    pub fn new() -> Self {
        let mut base = Box::new(Fixture::default());
        let mut arena_ = Box::new(TypeArena::default());
        let arena = arena_.as_mut() as *mut TypeArena;

        let mut builtin_types = Box::new(BuiltinTypes::new());
        let builtin_types_ptr = builtin_types.as_mut() as *mut BuiltinTypes;
        base.builtin_types = builtin_types_ptr;

        let mut ice_reporter = Box::new(InternalErrorReporter::default());
        let ice_reporter_ptr = ice_reporter.as_mut() as *mut InternalErrorReporter;

        let mut limits = Box::new(TypeCheckLimits::default());
        let limits_ptr = limits.as_mut() as *mut TypeCheckLimits;

        let base_ice_ptr = &mut base.ice as *mut InternalErrorReporter;
        let mut shared_state = Box::new(UnifierSharedState::unifier_shared_state(base_ice_ptr));
        let shared_state_ptr = shared_state.as_mut() as *mut UnifierSharedState;

        let solver_mode = if !FFlag::DebugLuauForceOldSolver.get() {
            SolverMode::New
        } else {
            SolverMode::Old
        };
        let mut normalizer = Box::new(Normalizer::new(
            arena,
            builtin_types_ptr,
            shared_state_ptr,
            solver_mode,
            false,
        ));
        let normalizer_ptr = normalizer.as_mut() as *mut Normalizer;

        let mut root_scope = Box::new(Scope::scope_type_pack_id(builtin_types.emptyTypePack));
        let root_scope_ptr = root_scope.as_mut() as *mut Scope;

        let runtime_root_scope = Arc::new(Scope::scope_type_pack_id(builtin_types.emptyTypePack));
        let mut type_function_runtime = Box::new(TypeFunctionRuntime::new(
            ice_reporter.as_ref(),
            limits.as_ref(),
            runtime_root_scope,
        ));
        let type_function_runtime_ptr = type_function_runtime.as_mut() as *mut TypeFunctionRuntime;

        let call_location = Location::default();
        let resolver = OverloadResolver::new(
            builtin_types_ptr,
            arena,
            normalizer_ptr,
            type_function_runtime_ptr,
            root_scope_ptr,
            ice_reporter_ptr,
            limits_ptr,
            call_location,
        );

        let mut k_empty_set = Box::new(DenseHashSet::new(core::ptr::null::<Type>() as TypeId));
        let empty_set = k_empty_set.as_mut() as *mut DenseHashSet<TypeId>;

        let k_dummy_location = Location::default();
        let k_dummy_expr = AstExprConstantNil::new(k_dummy_location);

        let number_type = builtin_types.numberType;
        let string_type = builtin_types.stringType;

        let number_to_number = add_function_type(arena, &[number_type], &[number_type]);
        let number_number_to_number =
            add_function_type(arena, &[number_type, number_type], &[number_type]);
        let number_to_string = add_function_type(arena, &[number_type], &[string_type]);
        let string_to_string = add_function_type(arena, &[string_type], &[string_type]);

        let number_to_number_and_string_to_string = unsafe {
            (*arena).add_type(IntersectionType {
                parts: alloc::vec![number_to_number, string_to_string],
            })
        };
        let number_to_number_and_number_number_to_number = unsafe {
            (*arena).add_type(IntersectionType {
                parts: alloc::vec![number_to_number, number_number_to_number],
            })
        };

        Self {
            arena_,
            arena,
            builtin_types,
            shared_state,
            normalizer,
            ice_reporter,
            limits,
            type_function_runtime,
            root_scope,
            call_location,
            resolver,
            k_empty_set,
            empty_set,
            k_dummy_location,
            k_dummy_expr,
            k_empty_exprs: alloc::vec::Vec::new(),
            number_to_number,
            number_number_to_number,
            number_to_string,
            string_to_string,
            number_to_number_and_string_to_string,
            number_to_number_and_number_number_to_number,
            base,
        }
    }
}

impl Default for OverloadResolverFixture {
    fn default() -> Self {
        Self::new()
    }
}

fn add_function_type(arena: *mut TypeArena, args: &[TypeId], rets: &[TypeId]) -> TypeId {
    unsafe {
        let arg_pack = (*arena).add_type_pack_initializer_list_type_id(args);
        let ret_pack = (*arena).add_type_pack_initializer_list_type_id(rets);
        (*arena).add_type(FunctionType::function_type_new(
            arg_pack, ret_pack, None, false,
        ))
    }
}
