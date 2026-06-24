use crate::records::module::Module;
use crate::type_aliases::scope_ptr_type::ScopePtr;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_stat::AstStat;
use luaur_common::records::dense_hash_map::DenseHashMap;
use luaur_config::records::lint_options::LintOptions;
use luaur_config::records::lint_warning::LintWarning;

#[derive(Debug, Clone)]
pub struct LintContext {
    pub result: alloc::vec::Vec<LintWarning>,
    pub options: LintOptions,
    pub root: *mut AstStat,
    pub placeholder: AstName,
    pub builtin_globals: DenseHashMap<AstName, crate::records::global_linter::Global>,
    pub scope: ScopePtr,
    pub module: *const Module,
}
