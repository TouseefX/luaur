//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:1592:pretty_printer_pretty_print_declare_global_stat`
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
//!   - translates_to -> rust_item pretty_printer_pretty_print_declare_global_stat

#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_declare_global_stat() {
    use crate::records::fixture::Fixture;
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;

    let _fixture = Fixture::default();
    let code = "declare _G: any";

    let mut options = ParseOptions::default();
    options.allow_declaration_syntax = true;

    let result = pretty_print_string_view_parse_options_bool_bool(code, options, true, false);
    assert_eq!(code, result.code);
}
