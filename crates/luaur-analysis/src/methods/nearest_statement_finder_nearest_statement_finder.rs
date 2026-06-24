use crate::records::nearest_statement_finder::NearestStatementFinder;
use luaur_ast::records::position::Position;

impl NearestStatementFinder {
    pub fn nearest_statement_finder_nearest_statement_finder(&mut self, cursor_position: Position) {
        self.cursor = cursor_position;
    }
}
