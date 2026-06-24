//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:122:ast_json_encoder_encode_tables`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - type_ref -> record AstStatBlock (Ast/include/Luau/Ast.h)
//!   - calls -> method JsonEncoderFixture::expectParse (tests/AstJsonEncoder.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeTable (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTableProp (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprTable (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantString (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstExprConstantNumber (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_tables

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_tables() {
    let mut fixture = JsonEncoderFixture::new();
    let src = r#"
        local x: {
            foo: number
        } = {
            foo = 123,
        }
    "#;
    let root = fixture.expect_parse(src);

    assert_eq!(
        json(root),
        const2(
            r#"{"type":"AstStatBlock","location":"0,0 - 6,4","hasEnd":true,"body":[{"type":"AstStatLocal","location":"1,8 - 5,9","vars":[{"luauType":{"type":"AstTypeTable","location":"1,17 - 3,9","props":[{"name":"foo","type":"AstTableProp","location":"2,12 - 2,15","propType":{"type":"AstTypeReference","location":"2,17 - 2,23","name":"number","nameLocation":"2,17 - 2,23","parameters":[]}}],"indexer":null},"name":"x","isConst":false,"type":"AstLocal","location":"1,14 - 1,15"}],"values":[{"type":"AstExprTable","location":"3,12 - 5,9","items":[{"type":"AstExprTableItem","kind":"record","key":{"type":"AstExprConstantString","location":"4,12 - 4,15","value":"foo"},"value":{"type":"AstExprConstantNumber","location":"4,18 - 4,21","value":123}}]}]}]}"#,
            r#"{"type":"AstStatBlock","location":"0,0 - 6,4","hasEnd":true,"body":[{"type":"AstStatLocal","location":"1,8 - 5,9","vars":[{"luauType":{"type":"AstTypeTable","location":"1,17 - 3,9","props":[{"name":"foo","type":"AstTableProp","location":"2,12 - 2,15","propType":{"type":"AstTypeReference","location":"2,17 - 2,23","name":"number","nameLocation":"2,17 - 2,23","parameters":[]}}],"indexer":null},"name":"x","type":"AstLocal","location":"1,14 - 1,15"}],"values":[{"type":"AstExprTable","location":"3,12 - 5,9","items":[{"type":"AstExprTableItem","kind":"record","key":{"type":"AstExprConstantString","location":"4,12 - 4,15","value":"foo"},"value":{"type":"AstExprConstantNumber","location":"4,18 - 4,21","value":123}}]}]}]}"#,
        )
    );
}
