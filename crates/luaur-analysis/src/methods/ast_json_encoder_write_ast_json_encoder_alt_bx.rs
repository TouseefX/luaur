//! Source: `Analysis/src/AstJsonEncoder.cpp:1037-1053` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_table_indexer::AstTableIndexer;

impl AstJsonEncoder {
    pub fn write_ast_table_indexer(&mut self, indexer: *mut AstTableIndexer) {
        if !indexer.is_null() {
            let i = unsafe { &*indexer };
            self.write_raw_string_view("{");
            let c = self.push_comma();
            self.write("location", &i.location);
            self.write("indexType", &i.index_type);
            self.write("resultType", &i.result_type);
            self.pop_comma(c);
            self.write_raw_string_view("}");
        } else {
            self.write_raw_string_view("null");
        }
    }
}
