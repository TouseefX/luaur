//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstJsonEncoder.test.cpp:500:ast_json_encoder_encode_ast_stat_declare_class`
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
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> method PathBuilder::prop (Analysis/src/TypePath.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - calls -> method SymDef::name (Analysis/include/Luau/ControlFlowGraph.h)
//!   - type_ref -> record AstTypeReference (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeFunction (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypeList (Ast/include/Luau/Ast.h)
//!   - type_ref -> type_alias AstArgumentName (Ast/include/Luau/Ast.h)
//!   - type_ref -> record AstTypePackExplicit (Ast/include/Luau/Ast.h)
//!   - translates_to -> rust_item ast_json_encoder_encode_ast_stat_declare_class

#[cfg(test)]
use super::ast_json_encoder_support::*;

#[cfg(test)]
#[test]
fn ast_json_encoder_encode_ast_stat_declare_class() {
    let mut fixture = JsonEncoderFixture::new();
    let root = fixture.expect_parse(
        r#"
        declare class Foo
            prop: number
            function method(self, foo: number): string
        end

        declare class Bar extends Foo
            prop2: string
        end
    "#,
    );

    assert_eq!(2, unsafe { (*root).body.size });

    assert_eq!(
        json(unsafe { block_statement(root, 0) }),
        r#"{"type":"AstStatDeclareClass","location":"1,22 - 4,11","name":"Foo","props":[{"name":"prop","nameLocation":"2,12 - 2,16","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeReference","location":"2,18 - 2,24","name":"number","nameLocation":"2,18 - 2,24","parameters":[]},"location":"2,12 - 2,24"},{"name":"method","nameLocation":"3,21 - 3,27","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeFunction","location":"3,12 - 3,54","attributes":[],"generics":[],"genericPacks":[],"argTypes":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"3,39 - 3,45","name":"number","nameLocation":"3,39 - 3,45","parameters":[]}]},"argNames":[{"type":"AstArgumentName","name":"foo","location":"3,34 - 3,37"}],"returnTypes":{"type":"AstTypePackExplicit","location":"3,48 - 3,54","typeList":{"type":"AstTypeList","types":[{"type":"AstTypeReference","location":"3,48 - 3,54","name":"string","nameLocation":"3,48 - 3,54","parameters":[]}]}}},"location":"3,12 - 3,54"}],"indexer":null}"#
    );
    assert_eq!(
        json(unsafe { block_statement(root, 1) }),
        r#"{"type":"AstStatDeclareClass","location":"6,22 - 8,11","name":"Bar","superName":"Foo","props":[{"name":"prop2","nameLocation":"7,12 - 7,17","type":"AstDeclaredClassProp","luauType":{"type":"AstTypeReference","location":"7,19 - 7,25","name":"string","nameLocation":"7,19 - 7,25","parameters":[]},"location":"7,12 - 7,25"}],"indexer":null}"#
    );
}
