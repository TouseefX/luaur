//! C++ `Frontend::parseType` (`Analysis/src/Frontend.cpp:2059-2183`).
use crate::enums::polarity::Polarity;
use crate::enums::solver_mode::SolverMode;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::constraint_graph::ConstraintGraph;
use crate::records::constraint_list::ConstraintList;
use crate::records::data_flow_graph_builder::DataFlowGraphBuilder;
use crate::records::frontend::Frontend;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module::Module;
use crate::records::module_info::ModuleInfo;
use crate::records::module_resolver::{ModuleResolver, ModuleResolverVtable};
use crate::records::normalizer::Normalizer;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::records::unifier_shared_state::UnifierSharedState;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::type_id::TypeId;
use alloc::rc::Rc;
use alloc::string::String;
use alloc::sync::Arc;
use alloc::vec::Vec;
use core::ptr::NonNull;
use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_name_table::AstNameTable;
use luaur_ast::records::parse_options::ParseOptions;
use luaur_ast::records::parser::Parser;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_common::{FFlag, FInt};

impl Frontend {
    pub fn parse_type(
        &mut self,
        allocator: &mut Allocator,
        name_table: &mut AstNameTable,
        ice_handler: &mut InternalErrorReporter,
        limits: TypeCheckLimits,
        arena: &mut TypeArena,
        source: &str,
    ) -> TypeId {
        let parse_result = Parser::parse_type_c_char_usize_ast_name_table_allocator_parse_options(
            source,
            name_table,
            allocator as *mut Allocator,
            ParseOptions::default(),
        );

        if parse_result.root.is_null() {
            ice_handler.ice_string("Frontend::parseType was given an unparseable type");
        }

        if let Some(error) = parse_result.errors.first() {
            ice_handler.ice_string(&alloc::format!(
                "Frontend::parseType error: {}",
                error.get_message()
            ));
        }

        let module: ModulePtr = Arc::new(Module::default());
        let module_ptr = Arc::as_ptr(&module) as *mut Module;

        let mut unifier_state = UnifierSharedState::unifier_shared_state(ice_handler);
        unifier_state.counters.recursion_limit = FInt::LuauTypeInferRecursionLimit.get() as i32;
        unifier_state.counters.iteration_limit = limits
            .unifierIterationLimit()
            .unwrap_or_else(|| FInt::LuauTypeInferIterationLimit.get() as i32);

        let mut normalizer = Normalizer::new(
            arena as *mut TypeArena,
            self.builtin_types,
            &mut unifier_state as *mut UnifierSharedState,
            SolverMode::New,
            false,
        );

        let mut type_function_runtime =
            TypeFunctionRuntime::new(ice_handler, &limits, self.globals.global_scope.clone());
        type_function_runtime.allow_evaluation = true;

        let mut module_resolver = null_module_resolver();

        let mut dfg = unsafe {
            DataFlowGraphBuilder::empty(&mut (*module_ptr).def_arena, &mut (*module_ptr).key_arena)
        };

        let mut cgraph_storage = if FFlag::LuauConstraintGraph.get() {
            Some(ConstraintGraph {
                builtin_types: NonNull::new(self.builtin_types as *mut BuiltinTypes)
                    .expect("builtinTypes must not be null"),
                dependencies: DenseHashMap::new(Default::default()),
                reverse_dependencies: DenseHashMap::new(Default::default()),
                constraint_lists: Vec::<Box<ConstraintList>>::new(),
            })
        } else {
            None
        };
        let cgraph = cgraph_storage
            .as_mut()
            .map(|cgraph| cgraph as *mut ConstraintGraph)
            .unwrap_or(core::ptr::null_mut());

        let prepare_module_scope: Rc<
            dyn Fn(&ModuleName, &crate::type_aliases::scope_ptr_type::ScopePtr),
        > = Rc::new(|_, _| {});

        let mut cg =
            crate::records::constraint_generator::ConstraintGenerator::constraint_generator(
                module,
                NonNull::new(&mut normalizer).unwrap(),
                NonNull::new(&mut type_function_runtime).unwrap(),
                NonNull::new(&mut module_resolver).unwrap(),
                NonNull::new(self.builtin_types).expect("builtinTypes must not be null"),
                NonNull::new(ice_handler).expect("iceHandler must not be null"),
                self.globals.global_scope.clone(),
                self.globals.global_scope.clone(),
                prepare_module_scope,
                core::ptr::null_mut(),
                NonNull::new(&mut dfg).unwrap(),
                Vec::new(),
                cgraph,
            );

        let ty = cg.resolve_type(
            Arc::as_ptr(&self.globals.global_scope) as *mut crate::records::scope::Scope,
            parse_result.root,
            false,
            false,
            Polarity::Positive,
        );

        if !cg.constraints.is_empty() {
            ice_handler
                .ice_string("Not yet implemented: parseType cannot reduce other type aliases");
        }

        ty
    }
}

fn null_module_resolver() -> ModuleResolver {
    ModuleResolver {
        vtable: ModuleResolverVtable {
            resolve_module_info: null_resolve_module_info,
            get_module: null_get_module,
            module_exists: null_module_exists,
            get_human_readable_module_name: null_get_human_readable_module_name,
        },
    }
}

unsafe fn null_resolve_module_info(
    _this: *mut ModuleResolver,
    _current_module_name: &ModuleName,
    _path_expr: *const AstExpr,
) -> Option<ModuleInfo> {
    None
}

unsafe fn null_get_module(
    _this: *const ModuleResolver,
    _module_name: &ModuleName,
) -> Option<ModulePtr> {
    None
}

unsafe fn null_module_exists(_this: *const ModuleResolver, _module_name: &ModuleName) -> bool {
    false
}

unsafe fn null_get_human_readable_module_name(
    _this: *const ModuleResolver,
    module_name: &ModuleName,
) -> String {
    module_name.clone()
}
