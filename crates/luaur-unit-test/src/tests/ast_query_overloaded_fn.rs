//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:67:ast_query_overloaded_fn`
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
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
//!   - calls -> method StringWriter::symbol (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item ast_query_overloaded_fn

#[cfg(test)]
#[test]
fn ast_query_overloaded_fn() {
    use crate::tests::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
        &String::from(
            r#"
        declare foo: ((string) -> number) & ((number) -> string)
    "#,
        ),
        false,
    );

    let symbol = fixture.get_doc_symbol(
        r#"
        foo("asdf")
    "#,
        Position {
            line: 1,
            column: 10,
        },
    );

    assert_eq!(
        symbol,
        Some(String::from("@test/global/foo/overload/(string) -> number"))
    );
}
