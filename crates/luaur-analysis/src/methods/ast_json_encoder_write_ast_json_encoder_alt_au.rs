//! Node: `cxx:Method:Luau.Analysis:Analysis/src/AstJsonEncoder.cpp:608:ast_json_encoder_write`
//! Source: `Analysis/src/AstJsonEncoder.cpp:608-647` (hand-ported)
use crate::records::ast_json_encoder::AstJsonEncoder;
use luaur_ast::records::ast_expr_binary::AstExprBinary_Op;
use luaur_common::macros::luau_assert::LUAU_ASSERT;

impl AstJsonEncoder {
    pub fn write_ast_expr_binary_op(&mut self, op: AstExprBinary_Op) {
        match op {
            AstExprBinary_Op::Add => self.write_string("Add"),
            AstExprBinary_Op::Sub => self.write_string("Sub"),
            AstExprBinary_Op::Mul => self.write_string("Mul"),
            AstExprBinary_Op::Div => self.write_string("Div"),
            AstExprBinary_Op::FloorDiv => self.write_string("FloorDiv"),
            AstExprBinary_Op::Mod => self.write_string("Mod"),
            AstExprBinary_Op::Pow => self.write_string("Pow"),
            AstExprBinary_Op::Concat => self.write_string("Concat"),
            AstExprBinary_Op::CompareNe => self.write_string("CompareNe"),
            AstExprBinary_Op::CompareEq => self.write_string("CompareEq"),
            AstExprBinary_Op::CompareLt => self.write_string("CompareLt"),
            AstExprBinary_Op::CompareLe => self.write_string("CompareLe"),
            AstExprBinary_Op::CompareGt => self.write_string("CompareGt"),
            AstExprBinary_Op::CompareGe => self.write_string("CompareGe"),
            AstExprBinary_Op::And => self.write_string("And"),
            AstExprBinary_Op::Or => self.write_string("Or"),
            _ => LUAU_ASSERT!(false),
        }
    }
}
