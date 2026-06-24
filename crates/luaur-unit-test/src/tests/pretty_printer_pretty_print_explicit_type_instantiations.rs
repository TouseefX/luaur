#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_explicit_type_instantiations() {
    use luaur_ast::functions::pretty_print_pretty_printer_alt_c::pretty_print_string_view_parse_options_bool_bool;
    use luaur_ast::functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parser::Parser;

    let mut code = "f<<A, B, C...>>() t.f<<A, B, C...>>() t:f<<A, B, C>>()" as &str;
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);

    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let mut parse_result = Parser::parse(
        code,
        code.len(),
        &mut names,
        &mut allocator,
        ParseOptions::default(),
    );
    assert!(parse_result.errors.is_empty(), "{:?}", parse_result.errors);
    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!(code, actual);

    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        false,
        false,
    );
    assert_eq!(
        "f              () t.f              () t:f           ()",
        result.code
    );

    code = "f < < A , B , C... > >( ) t.f < < A, B, C... > >  ( )  t:f< < A, B, C > > ( )";
    let result = pretty_print_string_view_parse_options_bool_bool(
        code,
        ParseOptions::default(),
        true,
        false,
    );
    assert_eq!(code, result.code);
}
