//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:1323:pretty_printer_pretty_print_mixed_union_intersection`
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
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - type_ref -> record Bar (tests/Variant.test.cpp)
//!   - translates_to -> rust_item pretty_printer_pretty_print_mixed_union_intersection

#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_mixed_union_intersection() {
    use crate::records::fixture::Fixture;
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;

    let _fixture = Fixture::default();
    for code in [
        "local a: string | (Foo & Bar)",
        "local a: string |   (Foo & Bar)",
        "local a: string | (  Foo & Bar)",
        "local a: string | (Foo & Bar  )",
        "local a: string &   (Foo | Bar)",
        "local a: string & (  Foo | Bar)",
        "local a: string & (Foo | Bar  )",
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
