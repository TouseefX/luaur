//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:594:ast_json_encoder_encode_ast_generic_type`
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
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstStatAssign (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprGlobal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprFunction (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstGenericType (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - calls -> method PathBuilder::args (Analysis/src/TypePath.cpp)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_generic_type

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_generic_type() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
        r#"
        a = function<b, c>()
        end
    "#,
    );

    assert_eq!(1, unsafe { (*root).body.size });

    assert_eq!(
        json(unsafe { block_statement(root, 0) }),
        r#"{"type":"AstStatAssign","location":"1,8 - 2,11","vars":[{"type":"AstExprGlobal","location":"1,8 - 1,9","global":"a"}],"values":[{"type":"AstExprFunction","location":"1,12 - 2,11","attributes":[],"generics":[{"type":"AstGenericType","name":"b"},{"type":"AstGenericType","name":"c"}],"genericPacks":[],"args":[],"vararg":false,"varargLocation":"0,0 - 0,0","body":{"type":"AstStatBlock","location":"1,28 - 2,8","hasEnd":true,"body":[]},"functionDepth":1,"debugname":""}]}"#
    );
}
