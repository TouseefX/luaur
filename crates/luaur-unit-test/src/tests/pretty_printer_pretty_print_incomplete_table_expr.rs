#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_incomplete_table_expr() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FFlag;

    let _fixture = Fixture::default();
    let _error_tolerant = ScopedFastFlag::new(&FFlag::LuauErrorTolerantPrettyPrinting, true);
    let _table_indent = ScopedFastFlag::new(&FFlag::LuauTableEntriesDontNeedToMatchIndent, true);

    let mut code = r#"local a = { a = 1 ["b"] = 2 }"#;
    let result =
        pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);

    code = r#"local a = { ["b" = 2, ["c"] 3, ["d" 4 }"#;
    let result =
        pretty_print_string_view_parse_options_bool_bool(code, ParseOptions::default(), true, true);
    assert_eq!(code, result.code);
}
