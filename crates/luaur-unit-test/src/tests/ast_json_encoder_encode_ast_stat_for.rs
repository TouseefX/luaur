//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:413:ast_json_encoder_encode_ast_stat_for`
//! Source: `tests/AstJsonEncoder.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/AstJsonEncoder.test.cpp
//! - source_includes:
//!   - includes -> source_file Ast/include/Luau/Ast.h
//!   - includes -> source_file Analysis/include/Luau/AstJsonEncoder.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/AstJsonEncoder.test.cpp
//! - outgoing:
//!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
//!   - calls -> method JsonEncoderFixture::expectParseStatement (tests/AstJsonEncoder.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstStatFor (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - calls -> method BcInstHelper::from (Bytecode/include/Luau/BytecodeOps.h)
//!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_for

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_stat_for() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("for a=0,1 do end");

    assert_eq!(
        json(statement),
        const2(
            r#"{"type":"AstStatFor","location":"0,0 - 0,16","var":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,4 - 0,5"},"from":{"type":"AstExprConstantNumber","location":"0,6 - 0,7","value":0},"to":{"type":"AstExprConstantNumber","location":"0,8 - 0,9","value":1},"body":{"type":"AstStatBlock","location":"0,12 - 0,13","hasEnd":true,"body":[]},"hasDo":true}"#,
            r#"{"type":"AstStatFor","location":"0,0 - 0,16","var":{"luauType":null,"name":"a","type":"AstLocal","location":"0,4 - 0,5"},"from":{"type":"AstExprConstantNumber","location":"0,6 - 0,7","value":0},"to":{"type":"AstExprConstantNumber","location":"0,8 - 0,9","value":1},"body":{"type":"AstStatBlock","location":"0,12 - 0,13","hasEnd":true,"body":[]},"hasDo":true}"#,
        )
    );
}
