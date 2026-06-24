//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:1347:pretty_printer_pretty_print_preserve_union_optional_style`
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
//!   - translates_to -> rust_item pretty_printer_pretty_print_preserve_union_optional_style

#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_preserve_union_optional_style() {
    use crate::records::fixture::Fixture;
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;

    let _fixture = Fixture::default();
    for code in [
        "local a: string | nil",
        "local a: string?",
        "local a: string???",
        "local a: string? | nil",
        "local a: string | nil | number",
        "local a: string | nil | number?",
        "local a: string? | number?",
    ] {
        let result = pretty_print_string_view_parse_options_bool_bool(
            code,
            ParseOptions::default(),
            true,
            false,
        );
        assert_eq!(code, result.code);
    }
}
