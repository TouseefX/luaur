use luaur_ast::records::ast_local::AstLocal;
use luaur_ast::records::ast_name::AstName;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::location::Location;
use luaur_common::records::dense_hash_map::DenseHashMap;

#[derive(Debug, Clone)]
pub struct FragmentAutocompleteAncestryResult {
    pub localMap: DenseHashMap<AstName, *mut AstLocal>,
    pub localStack: alloc::vec::Vec<*mut AstLocal>,
    pub ancestry: alloc::vec::Vec<*mut AstNode>,
    pub nearestStatement: *mut AstStat,
    pub parentBlock: *mut AstStatBlock,
    pub fragmentSelectionRegion: Location,
}
