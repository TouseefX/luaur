//! Generated skeleton item. @skeleton-stub
//! Node: `cxx:Test:Luau.UnitTest:tests/PrettyPrinter.test.cpp:96:pretty_printer_elseif_chains_indent_sensibly`
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
//!   - translates_to -> rust_item pretty_printer_elseif_chains_indent_sensibly

#[cfg(test)]
#[test]
fn pretty_printer_elseif_chains_indent_sensibly() {
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;

    let code = r#"
        if This then
            Once()
        elseif That then
            Another()
        elseif SecondLast then
            Third()
        else
            IfAllElseFails()
        end
    "#;

    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
    );
    assert_eq!(code, result.code);
}
