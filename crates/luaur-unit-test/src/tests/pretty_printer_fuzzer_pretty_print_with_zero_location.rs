#[cfg(test)]
#[test]
fn pretty_printer_fuzzer_pretty_print_with_zero_location() {
    use luaur_ast::functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parser::Parser;

    let example = r#"
if _ then
elseif _ then
elseif l0 then
else
local function l0<t0>(...):(t0<t0...>,(any)|(<t0>((any)|(<t0>(""[[[[[[[[[[[[[[[[[[[[[[[[!*t")->()))->()))
end
end
"#;

    let mut parse_options = ParseOptions::default();
    parse_options.capture_comments = true;

    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let mut parse_result = Parser::parse(
        example,
        example.len(),
        &mut names,
        &mut allocator,
        parse_options,
    );

    assert!(!parse_result.root.is_null());
    let _ = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
}
