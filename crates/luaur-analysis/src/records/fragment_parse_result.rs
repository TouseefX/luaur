use luaur_ast::records::allocator::Allocator;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::comment::Comment;
use luaur_ast::records::position::Position;

#[derive(Debug)]
pub struct FragmentParseResult {
    pub fragment_to_parse: alloc::string::String,
    pub root: *mut AstStatBlock,
    pub ancestry: alloc::vec::Vec<*mut AstNode>,
    pub nearest_statement: *mut AstStat,
    pub comment_locations: alloc::vec::Vec<Comment>,
    pub alloc: alloc::boxed::Box<Allocator>,
    pub scope_pos: Position,
}
