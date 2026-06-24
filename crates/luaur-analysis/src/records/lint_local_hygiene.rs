use crate::records::global_linter_alt_c::Global;
use crate::records::lint_context::LintContext;
use crate::records::local_linter::Local;
use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_name::AstName;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct LintLocalHygiene {
    pub(crate) context: *mut LintContext,
    pub(crate) locals: DenseHashMap<*mut AstLocal, Local>,
    pub(crate) imports: DenseHashMap<AstName, *mut AstLocal>,
    pub(crate) globals: DenseHashMap<AstName, Global>,
}
