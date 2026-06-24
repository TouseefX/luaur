//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:38:ast_query_prop`
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
//!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item ast_query_prop

#[cfg(test)]
#[test]
fn ast_query_prop() {
    use crate::tests::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    let substring = fixture.get_doc_symbol(
        r#"
        local a = string.sub()
    "#,
        Position {
            line: 1,
            column: 27,
        },
    );

    assert_eq!(substring, Some(String::from("@luau/global/string.sub")));
}
