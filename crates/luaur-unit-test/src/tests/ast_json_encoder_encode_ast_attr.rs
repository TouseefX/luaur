//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:488:ast_json_encoder_encode_ast_attr`
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
//!   - type_ref -> record AstStatFunction (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstAttr (Ast/include/Luau/Ast.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatReturn (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_attr

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_attr() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_statement("@checked function a(b) return c end");

    assert_eq!(
        json(expr),
        const2(
            r#"{"type":"AstStatFunction","location":"0,0 - 0,35","name":{"type":"AstExprGlobal","location":"0,18 - 0,19","global":"a"},"func":{"type":"AstExprFunction","location":"0,0 - 0,35","attributes":[{"type":"AstAttr","location":"0,0 - 0,8","name":"checked"}],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"b","isConst":false,"type":"AstLocal","location":"0,20 - 0,21"}],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,22 - 0,32","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,23 - 0,31","list":[{"type":"AstExprGlobal","location":"0,30 - 0,31","global":"c"}]}]},"functionDepth":1,"debugname":"a"}}"#,
            r#"{"type":"AstStatFunction","location":"0,0 - 0,35","name":{"type":"AstExprGlobal","location":"0,18 - 0,19","global":"a"},"func":{"type":"AstExprFunction","location":"0,0 - 0,35","attributes":[{"type":"AstAttr","location":"0,0 - 0,8","name":"checked"}],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"b","type":"AstLocal","location":"0,20 - 0,21"}],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,22 - 0,32","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,23 - 0,31","list":[{"type":"AstExprGlobal","location":"0,30 - 0,31","global":"c"}]}]},"functionDepth":1,"debugname":"a"}}"#,
        )
    );
}
