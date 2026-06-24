//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/AstQuery.test.cpp:50:ast_query_event_callback_arg`
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
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> type_alias DocumentationSymbol (Analysis/include/Luau/Documentation.h)
//!   - calls -> method DocumentationSymbolFixture::getDocSymbol (tests/AstQuery.test.cpp)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - calls -> method AssemblyBuilderX64::test (CodeGen/src/AssemblyBuilderX64.cpp)
//!   - translates_to -> rust_item ast_query_event_callback_arg

#[cfg(test)]
#[test]
fn ast_query_event_callback_arg() {
    use crate::tests::ast_query_support::*;

    let mut fixture = DocumentationSymbolFixture::default();
    fixture.base.base.load_definition(
        &String::from(
            r#"
        declare function Connect(fn: (string) -> ())
    "#,
        ),
        false,
    );

    let substring = fixture.get_doc_symbol(
        r#"
        Connect(function(abc)
        end)
    "#,
        Position {
            line: 1,
            column: 27,
        },
    );

    assert_eq!(
        substring,
        Some(String::from("@test/global/Connect/param/0/param/0"))
    );
}
