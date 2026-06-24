use crate::records::builtin_types::BuiltinTypes;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::normalizer::Normalizer;
use crate::records::overload_resolution::OverloadResolution;
use crate::records::overload_resolver;
use crate::records::scope::Scope;
use crate::records::subtyping::Subtyping;
use crate::records::subtyping_reasoning::SubtypingReasoning;
use crate::records::type_arena::TypeArena;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::type_aliases::error_vec::ErrorVec;
use crate::type_aliases::module_name_type::ModuleName;
use crate::type_aliases::type_id::TypeId;
use crate::type_aliases::type_or_pack::TypeOrPack;
use crate::type_aliases::type_pack_id::TypePackId;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug, Clone)]
pub struct OverloadResolver {
    pub(crate) builtin_types: *mut BuiltinTypes,
    pub(crate) arena: *mut TypeArena,
    pub(crate) normalizer: *mut Normalizer,
    pub(crate) type_function_runtime:
        *mut crate::records::type_function_runtime::TypeFunctionRuntime,
    pub(crate) scope: *mut Scope,
    pub(crate) ice: *mut InternalErrorReporter,
    pub(crate) limits: TypeCheckLimits,
    pub(crate) subtyping: Subtyping,
    pub(crate) call_loc: Location,
}
