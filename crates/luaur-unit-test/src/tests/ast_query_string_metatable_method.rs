//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:166:ast_query_string_metatable_method`
//! Source: `tests/AstQuery.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/AstQuery.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/AstQuery.h
//!   - includes -> source_file tests/AstQueryDsl.h
//!   - includes -> source_file tests/ClassFixture.h
//! - incoming:
//!   - declares <- source_file tests/AstQuery.test.cpp
//! - outgoing:
//!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
//!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - calls -> function rep (tests/Fixture.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item ast_query_string_metatable_method

#[cfg(test)]
#[test]
fn ast_query_string_metatable_method() {
    use crate::tests::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    let symbol = fixture.get_doc_symbol(
        r#"
        local x: string = "Foo"
        x:rep(2)
    "#,
        Position {
            line: 2,
            column: 12,
        },
    );

    assert_eq!(symbol, Some(String::from("@luau/global/string.rep")));
}
