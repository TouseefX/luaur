#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_incomplete_repeat_stat() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FFlag;

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&FFlag::LuauErrorTolerantPrettyPrinting, true);
    let code = r#"
repeat
    print("hello world")
"#;
    let expected = r#"
repeat
    print("hello world")
(error-expr)"#;

    let result =
        pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(expected, result.code);
}
