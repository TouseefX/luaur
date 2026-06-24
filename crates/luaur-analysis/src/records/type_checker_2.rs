//! Node: TypeChecker2 record
//! Source: `Analysis/include/Luau/TypeChecker2.h` (hand-ported; fields only)

use crate::enums::type_context::TypeContext;
use crate::records::builtin_types::BuiltinTypes;
use crate::records::dcr_logger::DcrLogger;
use crate::records::internal_error_reporter::InternalErrorReporter;
use crate::records::module::Module;
use crate::records::normalizer::Normalizer;
use crate::records::scope::Scope;
use crate::records::source_module::SourceModule;
use crate::records::subtyping::Subtyping;
use crate::records::type_check_limits::TypeCheckLimits;
use crate::records::type_function_runtime::TypeFunctionRuntime;
use crate::type_aliases::type_id::TypeId;
use alloc::string::String;
use alloc::vec::Vec;
use luaur_common::records::dense_hash_set::DenseHashSet;

#[derive(Debug)]
pub struct TypeChecker2 {
    pub builtin_types: *mut BuiltinTypes,
    pub type_function_runtime: *mut TypeFunctionRuntime,
    pub logger: *mut DcrLogger,
    pub limits: *mut TypeCheckLimits,
    pub ice: *mut InternalErrorReporter,
    pub source_module: *const SourceModule,
    pub module: *mut Module,
    pub type_context: TypeContext,

    pub stack: Vec<*mut Scope>,
    pub function_decl_stack: Vec<TypeId>,

    pub seen_type_function_instances: DenseHashSet<TypeId>,

    pub normalizer: Normalizer,
    pub _subtyping: Subtyping,
    pub subtyping: *mut Subtyping,

    pub warned_globals: DenseHashSet<String>,
}
