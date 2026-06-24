use crate::records::json_encoder_fixture::JsonEncoderFixture;
use alloc::string::String;
use alloc::string::ToString;
use luaur_ast::records::ast_expr::AstExpr;
use luaur_ast::records::ast_stat::AstStat;
use luaur_ast::records::ast_stat_assign::AstStatAssign;
use luaur_ast::records::ast_stat_block::AstStatBlock;

impl JsonEncoderFixture {
    pub fn expect_parse_expr(&mut self, src: &str) -> *mut AstExpr {
        let mut s = String::from("a = ");
        s.push_str(src);
        let root = self.expect_parse(&s);
        let root_ref = unsafe { &*root };

        luaur_common::LUAU_ASSERT!(root_ref.body.size > 0);
        let stat = unsafe { *root_ref.body.data.add(0) };

        let stat_assign = unsafe {
            luaur_ast::rtti::ast_node_as::<AstStatAssign>(
                stat as *mut luaur_ast::records::ast_node::AstNode,
            )
        };
        luaur_common::LUAU_ASSERT!(!stat_assign.is_null());

        let stat_assign_ref = unsafe { &*stat_assign };
        luaur_common::LUAU_ASSERT!(stat_assign_ref.values.size == 1);

        unsafe { *stat_assign_ref.values.data.add(0) }
    }
}
