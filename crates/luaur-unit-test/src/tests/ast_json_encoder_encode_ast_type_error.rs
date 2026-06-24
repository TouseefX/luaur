//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:564:ast_json_encoder_encode_ast_type_error`
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
//!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
//!   - calls -> method JsonEncoderFixture::parse (tests/AstJsonEncoder.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstStat (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTypeError (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_type_error

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_type_error() {
    let mut fixture = JsonEncoderFixture::new();
    let parse_result = fixture.parse("type T = ");
    assert_eq!(1, unsafe { (*parse_result.root).body.size });

    let statement = unsafe { block_statement(parse_result.root, 0) };

    assert_eq!(
        json(statement),
        r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,9","name":"T","generics":[],"genericPacks":[],"value":{"type":"AstTypeError","location":"0,8 - 0,9","types":[],"messageIndex":0},"exported":false}"#
    );
}
