use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_block::AstStatBlock;
use luaur_ast::records::ast_visitor::AstVisitor;
use luaur_ast::records::location::Location;
use luaur_ast::records::position::Position;

#[derive(Debug, Clone)]
pub struct NearestStatementFinder {
    pub(crate) cursor: Position,
    pub(crate) nearest: *mut AstStat,
    pub(crate) parent: *mut AstStatBlock,
}

impl NearestStatementFinder {
    pub fn new(cursor_position: Position) -> Self {
        Self {
            cursor: cursor_position,
            nearest: core::ptr::null_mut(),
            parent: core::ptr::null_mut(),
        }
    }
}

impl NearestStatementFinder {
    pub fn nearest_statement_finder(cursor_position: &Position) -> Self {
        Self::new(*cursor_position)
    }
}

impl AstVisitor for NearestStatementFinder {
    fn visit_stat_block(&mut self, node: *mut core::ffi::c_void) -> bool {
        let block = node as *mut AstStatBlock;
        unsafe {
            let block_location = (*block).base.base.location;
            if block_location.contains(self.cursor) || block_location.end == self.cursor {
                self.parent = block;

                // Find last statement whose begin <= cursor.
                for v in (*block).body.iter() {
                    let v = *v;
                    let stmt_location = unsafe { (*v).base.location };
                    if stmt_location.begin <= self.cursor {
                        self.nearest = v;
                    }
                }

                true
            } else {
                false
            }
        }
    }
}
