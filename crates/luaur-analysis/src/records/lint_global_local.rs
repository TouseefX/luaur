use crate::records::function_info::FunctionInfo;
use crate::records::global_linter_alt_b::Global;
use crate::records::lint_context::LintContext;
use alloc::vec::Vec;
use luaur_ast::records::ast_expr_global::AstExprGlobal;
use luaur_ast::records::ast_name::AstName;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct LintGlobalLocal {
    pub(crate) context: *mut LintContext,
    pub(crate) globals: DenseHashMap<AstName, Global>,
    pub(crate) global_refs: Vec<*mut AstExprGlobal>,
    pub(crate) function_stack: Vec<FunctionInfo>,
}
