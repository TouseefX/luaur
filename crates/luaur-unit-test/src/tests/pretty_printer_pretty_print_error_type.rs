#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_error_type() {
    use crate::records::fixture::Fixture;
    use luaur_ast::functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parser::Parser;

    let _fixture = Fixture::default();
    let code = "local a: ";

    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let mut parse_result = Parser::parse(
        code,
        code.len(),
        &mut names,
        &mut allocator,
        ParseOptions::default(),
    );

    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!("local a:%error-type%", actual);
}
