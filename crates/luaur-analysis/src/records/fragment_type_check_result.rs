use luaur_ast::records::ast_node::AstNode;

use crate::type_aliases::module_ptr_module::ModulePtr;
use crate::type_aliases::scope_ptr_anyification::ScopePtr;

#[derive(Debug, Clone)]
pub struct FragmentTypeCheckResult {
    pub incremental_module: Option<ModulePtr>,
    pub fresh_scope: ScopePtr,
    pub ancestry: alloc::vec::Vec<*mut AstNode>,
}
