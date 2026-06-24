use crate::records::fixture::Fixture;
use alloc::boxed::Box;
use luaur_analysis::records::builtin_types::BuiltinTypes;
use luaur_analysis::records::internal_error_reporter::InternalErrorReporter;
use luaur_analysis::records::normalizer::Normalizer;
use luaur_analysis::records::overload_resolver::OverloadResolver;
use luaur_analysis::records::scope::Scope;
use luaur_analysis::records::type_arena::TypeArena;
use luaur_analysis::records::type_check_limits::TypeCheckLimits;
use luaur_analysis::records::type_function_runtime::TypeFunctionRuntime;
use luaur_analysis::records::unifier_shared_state::UnifierSharedState;
use luaur_analysis::type_aliases::type_id::TypeId;
use luaur_analysis::type_aliases::type_pack_id::TypePackId;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_expr_constant_nil::AstExprConstantNil;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug)]
#[repr(C)]
pub struct OverloadResolverFixture {
    pub arena_: Box<TypeArena>,
    pub arena: *mut TypeArena,
    pub builtin_types: Box<BuiltinTypes>,
    pub shared_state: Box<UnifierSharedState>,
    pub normalizer: Box<Normalizer>,
    pub ice_reporter: Box<InternalErrorReporter>,
    pub limits: Box<TypeCheckLimits>,
    pub type_function_runtime: Box<TypeFunctionRuntime>,
    pub root_scope: Box<Scope>,
    pub call_location: Location,
    pub resolver: OverloadResolver,
    pub k_empty_set: Box<DenseHashSet<TypeId>>,
    pub empty_set: *mut DenseHashSet<TypeId>,
    pub k_dummy_location: Location,
    pub k_dummy_expr: AstExprConstantNil,
    pub k_empty_exprs: alloc::vec::Vec<*mut AstExpr>,
    pub number_to_number: TypeId,
    pub number_number_to_number: TypeId,
    pub number_to_string: TypeId,
    pub string_to_string: TypeId,
    pub number_to_number_and_string_to_string: TypeId,
    pub number_to_number_and_number_number_to_number: TypeId,
    pub base: Box<Fixture>,
}
