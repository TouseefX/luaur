use crate::functions::block_diff_start::block_diff_start;
use crate::functions::get_fragment_location::get_fragment_location;
use crate::records::fragment_region::FragmentRegion;
use crate::records::nearest_likely_block_finder::NearestLikelyBlockFinder;
use crate::records::nearest_statement_finder::NearestStatementFinder;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;
use luaur_ast::visit::ast_stat_block_visit;

pub fn get_fragment_region_with_block_diff(
    stale: *mut AstStatBlock,
    fresh: *mut AstStatBlock,
    cursor_pos: &Position,
) -> FragmentRegion {
    let mut nsf = NearestStatementFinder::new(*cursor_pos);
    ast_stat_block_visit(unsafe { &*fresh }, &mut nsf);

    let parent = if !nsf.parent.is_null() {
        nsf.parent
    } else {
        fresh
    };

    let nearest = if !nsf.nearest.is_null() {
        nsf.nearest
    } else {
        fresh as *mut AstStat
    };

    let mut lsf = NearestLikelyBlockFinder::nearest_likely_block_finder(parent);
    // C++ `stale->visit(&lsf)` — traverse the entire stale AST so the visitor
    // sees every nested block (e.g. the inner `do` block), not just the root.
    ast_stat_block_visit(unsafe { &*stale }, &mut lsf);

    if let Some(same_block) = lsf.found {
        if let Some(fd) = block_diff_start(same_block, parent, nearest) {
            return FragmentRegion {
                fragment_location: Location::new(fd, *cursor_pos),
                nearest_statement: nearest,
                parent_block: parent,
            };
        }
    }

    FragmentRegion {
        fragment_location: get_fragment_location(nsf.nearest, cursor_pos),
        nearest_statement: nearest,
        parent_block: parent,
    }
}
