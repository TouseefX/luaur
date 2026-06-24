use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_map::DenseHashMap;

use crate::records::module_info::ModuleInfo;
use crate::type_aliases::module_name_file_resolver::ModuleName;

#[derive(Debug, Clone)]
pub struct RequireTraceResult {
    pub exprs: DenseHashMap<*mut AstNode, ModuleInfo>,
    pub require_list: alloc::vec::Vec<(ModuleName, Location)>,
}
