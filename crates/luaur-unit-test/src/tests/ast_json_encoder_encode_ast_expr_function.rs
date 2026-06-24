//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:293:ast_json_encoder_encode_ast_expr_function`
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
//!   - type_ref -> record AstExpr (Ast/include/Luau/Ast.h)
//!   - calls -> method JsonEncoderFixture::expectParseExpr (tests/AstJsonEncoder.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatReturn (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprLocal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_expr_function

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_expr_function() {
    let mut fixture = JsonEncoderFixture::new();
    let expr = fixture.expect_parse_expr("function (a) return a end");

    assert_eq!(
        json(expr),
        const2(
            r#"{"type":"AstExprFunction","location":"0,4 - 0,29","attributes":[],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,14 - 0,15"}],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,16 - 0,26","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,17 - 0,25","list":[{"type":"AstExprLocal","location":"0,24 - 0,25","local":{"luauType":null,"name":"a","isConst":false,"type":"AstLocal","location":"0,14 - 0,15"}}]}]},"functionDepth":1,"debugname":""}"#,
            r#"{"type":"AstExprFunction","location":"0,4 - 0,29","attributes":[],"generics":[],"genericPacks":[],"args":[{"luauType":null,"name":"a","type":"AstLocal","location":"0,14 - 0,15"}],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"0,16 - 0,26","hasEnd":true,"body":[{"type":"AstStatReturn","location":"0,17 - 0,25","list":[{"type":"AstExprLocal","location":"0,24 - 0,25","local":{"luauType":null,"name":"a","type":"AstLocal","location":"0,14 - 0,15"}}]}]},"functionDepth":1,"debugname":""}"#,
        )
    );
}
