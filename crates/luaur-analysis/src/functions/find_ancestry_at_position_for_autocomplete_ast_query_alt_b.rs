use crate::records::autocomplete_node_finder::AutocompleteNodeFinder;
use alloc::vec::Vec;
use luaur_ast::records::ast_node::AstNode;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::position::Position;
use luaur_ast::visit::AstVisitable;

pub fn find_ancestry_at_position_for_autocomplete_ast_stat_block_position(
    root: &mut AstStatBlock,
    pos: Position,
) -> Vec<*mut AstNode> {
    let mut finder = AutocompleteNodeFinder::new(pos);
    root.visit(&mut finder);
    finder.ancestry
}
