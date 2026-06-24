use crate::functions::get_fragment_location::get_fragment_location;
use crate::records::fragment_region::FragmentRegion;
use crate::records::nearest_statement_finder::NearestStatementFinder;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::position::Position;
use luaur_ast::visit::ast_stat_block_visit;

pub fn get_fragment_region(root: *mut AstStatBlock, cursor_position: &Position) -> FragmentRegion {
    let mut nsf = NearestStatementFinder::new(*cursor_position);
    ast_stat_block_visit(unsafe { &*root }, &mut nsf);

    let parent = if !nsf.parent.is_null() {
        nsf.parent
    } else {
        root
    };

    FragmentRegion {
        fragment_location: get_fragment_location(nsf.nearest, cursor_position),
        nearest_statement: nsf.nearest,
        parent_block: parent,
    }
}
