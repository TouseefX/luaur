//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:1986:pretty_printer_pretty_print_type_function_named_arguments`
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
//!   - calls -> function first (Analysis/src/TypePack.cpp)
//!   - translates_to -> rust_item pretty_printer_pretty_print_type_function_named_arguments

#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_type_function_named_arguments() {
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;

    for code in [
        r#" type Foo = (x: string) -> () "#,
        r#" type Foo = (x: string, y: number) -> ()  "#,
        r#" type Foo = (  x: string, y: number) -> () "#,
        r#" type Foo = (x  : string, y: number) -> () "#,
        r#" type Foo = (x:   string, y: number) -> () "#,
        r#" type Foo = (x: string,   y: number) -> () "#,
        r#" type Foo = (number, info: string) -> () "#,
        r#" type Foo = (first: string, second: string, ...string) -> () "#,
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
