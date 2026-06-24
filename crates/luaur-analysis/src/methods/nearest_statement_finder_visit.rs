use crate::records::nearest_statement_finder::NearestStatementFinder;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

impl NearestStatementFinder {
    pub fn visit(&mut self, block: *mut AstStatBlock) -> bool {
        let block_ref = unsafe { &*block };
        let block_location: Location = block_ref.base.base.location;

        if block_location.contains(self.cursor) {
            self.parent = block;

            let body = block_ref.body;
            for i in 0..body.size {
                let stat = unsafe { *body.data.add(i) };
                let stat_ref = unsafe { &*stat };
                let stat_location = stat_ref.base.location;

                if stat_location.begin <= self.cursor {
                    self.nearest = stat;
                }
            }

            true
        } else {
            false
        }
    }
}
