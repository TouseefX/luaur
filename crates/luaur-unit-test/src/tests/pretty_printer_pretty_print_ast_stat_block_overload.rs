//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:40:pretty_printer_pretty_print_ast_stat_block_overload`
//! Source: `tests/PrettyPrinter.test.cpp`
//! Graph edges:
//! - declared_by: source_file tests/PrettyPrinter.test.cpp
//! - source_includes:
//!   - includes -> source_file Common/include/Luau/Common.h
//!   - includes -> source_file Ast/include/Luau/Parser.h
//!   - includes -> source_file Ast/include/Luau/PrettyPrinter.h
//!   - includes -> source_file tests/ClassFixture.h
//!   - includes -> source_file tests/ScopedFlags.h
//! - incoming:
//!   - declares <- source_file tests/PrettyPrinter.test.cpp
//! - outgoing:
//!   - calls -> method StringWriter::string (Ast/src/PrettyPrinter.cpp)
//!   - calls -> method TypeError::code (Analysis/src/Error.cpp)
//!   - type_ref -> record ParseOptions (Ast/include/Luau/ParseOptions.h)
//!   - type_ref -> record Allocator (Ast/include/Luau/Allocator.h)
//!   - type_ref -> record AstNameTable (Ast/include/Luau/Lexer.h)
//!   - type_ref -> record ParseResult (Ast/include/Luau/ParseResult.h)
//!   - type_ref -> record Parser (Ast/include/Luau/Parser.h)
//!   - calls -> method Symbol::c_str (Analysis/include/Luau/Symbol.h)
//!   - translates_to -> rust_item pretty_printer_pretty_print_ast_stat_block_overload

#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_ast_stat_block_overload() {
    use luaur_ast::functions::pretty_print_pretty_printer_alt_b::pretty_print_ast_stat_block;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parser::Parser;

    let code = "local a = 1";
    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let mut result = Parser::parse(
        code,
        code.len(),
        &mut names,
        &mut allocator,
        ParseOptions::default(),
    );

    assert!(!result.root.is_null());

    let printed = unsafe { pretty_print_ast_stat_block(&mut *result.root) };
    assert_eq!("local a = 1", printed);
}
