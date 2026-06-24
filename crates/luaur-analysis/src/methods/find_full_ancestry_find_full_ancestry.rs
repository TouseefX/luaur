use crate::records::find_full_ancestry::FindFullAncestry;
use luaur_ast::records::position::Position;

impl FindFullAncestry {
    pub fn new(pos: Position, document_end: Position, include_types: bool) -> Self {
        FindFullAncestry {
            nodes: Vec::new(),
            pos,
            document_end,
            include_types,
        }
    }
}
