use crate::records::json_encoder_fixture::JsonEncoderFixture;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl JsonEncoderFixture {
    pub fn expect_parse(&mut self, src: &str) -> *mut AstStatBlock {
        let parse_result = self.parse(src);
        luaur_common::LUAU_ASSERT!(parse_result.errors.len() == 0);
        parse_result.root
    }
}
