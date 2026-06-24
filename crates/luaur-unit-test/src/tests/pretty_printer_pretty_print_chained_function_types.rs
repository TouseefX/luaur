//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:2091:pretty_printer_pretty_print_chained_function_types`
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
//!   - calls -> type_alias type (Common/include/Luau/Variant.h)
//!   - type_ref -> record Foo (tests/Variant.test.cpp)
//!   - translates_to -> rust_item pretty_printer_pretty_print_chained_function_types

#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_chained_function_types() {
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;

    for code in [
        r#" type Foo = () -> () -> () "#,
        r#" type Foo = () -> ()   -> () "#,
        r#" type Foo = () -> () ->   () "#,
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
