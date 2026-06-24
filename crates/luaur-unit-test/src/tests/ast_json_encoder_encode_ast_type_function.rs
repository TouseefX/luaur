//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:555:ast_json_encoder_encode_ast_type_function`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record AstStatTypeAlias (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTypeFunction (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - type_ref -> type_alias AstArgumentName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_type_function

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_type_function() {
    let mut fixture = JsonEncoderFixture::new();
    let statement =
        fixture.expect_parse_statement(r#"type fun = (string, bool, named: number) -> ()"#);

    assert_eq!(
        json(statement),
        r#"{"type":"AstStatTypeAlias","location":"0,0 - 0,46","name":"fun","generics":[],"genericPacks":[],"value":{"type":"AstTypeFunction","location":"0,11 - 0,46","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"0,12 - 0,18","name":"string","nameLocation":"0,12 - 0,18","parameters":[]},{"type":"AstTypeReference","location":"0,20 - 0,24","name":"bool","nameLocation":"0,20 - 0,24","parameters":[]},{"type":"AstTypeReference","location":"0,33 - 0,39","name":"number","nameLocation":"0,33 - 0,39","parameters":[]}]},"argNames":[null,null,{"type":"AstArgumentName","name":"named","location":"0,26 - 0,31"}],"returnTypes":{"type":"AstTypePackExplicit","location":"0,44 - 0,46","typeList":{"type":"AstTypeList","types":[]}}},"exported":false}"#
    );
}
