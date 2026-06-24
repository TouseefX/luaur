//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/Module.test.cpp:48:module_is_within_comment_parse_result`
//! Source: `tests/Module.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/Module.test.cpp
//! - source_includes:
//!   - includes -> source_file Analysis/include/Luau/Clone.h
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Analysis/include/Luau/Module.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/Module.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> function foo (tests/NotNull.test.cpp)
//!   - calls -> function bar (tests/NotNull.test.cpp)
//!   - calls -> method StringWriter::space (Ast/src/PrettyPrinter.cpp)
//!   - type_ref -> record Allocator (Ast/include/Luau/Allocator.h)
//!   - type_ref -> record AstNameTable (Ast/include/Luau/Lexer.h)
//!   - type_ref -> record ParseOptions (Ast/include/Luau/ParseOptions.h)
//!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
//!   - type_ref -> record Parser (Ast/include/Luau/Parser.h)
//!   - type_ref -> record Position (Ast/include/Luau/Location.h)
//!   - translates_to -> rust_item module_is_within_comment_parse_result

#[cfg(test)]
#[test]
fn module_is_within_comment_parse_result() {
    use luaur_analysis::functions::is_within_comment_module_alt_c::is_within_comment_parse_result_position;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parser::Parser;
    use luaur_ast::records::position::Position;

    let src = alloc::string::String::from(
        r#"
        --!strict
        local foo = {}
        function foo:bar() end

        --[[
            foo:
        ]] foo:bar()

        --[[]]--[[]] -- Two distinct comments that have zero characters of space between them.
    "#,
    );

    let mut alloc = Allocator::allocator();
    let mut names = AstNameTable::new(&mut alloc);
    let mut parse_options = ParseOptions::default();
    parse_options.capture_comments = true;
    let parse_result = Parser::parse(&src, src.len(), &mut names, &mut alloc, parse_options);

    assert_eq!(5, parse_result.comment_locations.len());

    assert!(is_within_comment_parse_result_position(
        &parse_result,
        Position::new(1, 15)
    ));
    assert!(is_within_comment_parse_result_position(
        &parse_result,
        Position::new(6, 16)
    ));
    assert!(is_within_comment_parse_result_position(
        &parse_result,
        Position::new(9, 13)
    ));
    assert!(is_within_comment_parse_result_position(
        &parse_result,
        Position::new(9, 14)
    ));

    assert!(!is_within_comment_parse_result_position(
        &parse_result,
        Position::new(2, 15)
    ));
    assert!(!is_within_comment_parse_result_position(
        &parse_result,
        Position::new(7, 10)
    ));
    assert!(!is_within_comment_parse_result_position(
        &parse_result,
        Position::new(7, 11)
    ));
}
