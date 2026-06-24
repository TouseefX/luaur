//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:577:ast_json_encoder_encode_ast_type_pack_explicit`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record AstStatLocal (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstLocal (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_type_pack_explicit

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_type_pack_explicit() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
        r#"
        type A<T...> = () -> T...
        local a: A<(number, string)>
    "#,
    );

    assert_eq!(2, unsafe { (*root).body.size });

    assert_eq!(
        json(unsafe { block_statement(root, 1) }),
        const2(
            r#"{"type":"AstStatLocal","location":"2,8 - 2,36","vars":[{"luauType":{"type":"AstTypeReference","location":"2,17 - 2,36","name":"A","nameLocation":"2,17 - 2,18","parameters":[{"type":"AstTypePackExplicit","location":"2,19 - 2,20","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"2,20 - 2,26","name":"number","nameLocation":"2,20 - 2,26","parameters":[]},{"type":"AstTypeReference","location":"2,28 - 2,34","name":"string","nameLocation":"2,28 - 2,34","parameters":[]}]}}]},"name":"a","isConst":false,"type":"AstLocal","location":"2,14 - 2,15"}],"values":[]}"#,
            r#"{"type":"AstStatLocal","location":"2,8 - 2,36","vars":[{"luauType":{"type":"AstTypeReference","location":"2,17 - 2,36","name":"A","nameLocation":"2,17 - 2,18","parameters":[{"type":"AstTypePackExplicit","location":"2,19 - 2,20","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"2,20 - 2,26","name":"number","nameLocation":"2,20 - 2,26","parameters":[]},{"type":"AstTypeReference","location":"2,28 - 2,34","name":"string","nameLocation":"2,28 - 2,34","parameters":[]}]}}]},"name":"a","type":"AstLocal","location":"2,14 - 2,15"}],"values":[]}"#,
        )
    );
}
