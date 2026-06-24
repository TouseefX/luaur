//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:469:ast_json_encoder_encode_ast_stat_declare_function`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstStatDeclareFunction (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - type_ref -> type_alias AstArgumentName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_declare_function

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_stat_declare_function() {
    let mut fixture = JsonEncoderFixture::new();
    let statement = fixture.expect_parse_statement("declare function foo(x: number): string");

    assert_eq!(
        json(statement),
        r#"{"type":"AstStatDeclareFunction","location":"0,0 - 0,39","attributes":[],"name":"foo","nameLocation":"0,17 - 0,20","params":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,24 - 0,30","name":"number","nameLocation":"0,24 - 0,30","parameters":[]}]},"paramNames":[{"type":"AstArgumentName","name":"x","location":"0,21 - 0,22"}],"vararg":false,"varargLocation":"0,0 - 0,0","retTypes":{"type":"AstTypePackExplicit","location":"0,33 - 0,39","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,33 - 0,39","name":"string","nameLocation":"0,33 - 0,39","parameters":[]}]}},"generics":[],"genericPacks":[]}"#
    );
}
