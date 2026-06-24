#[cfg(test)]
#[test]
fn pretty_printer_roundtrip_types() {
    use luaur_ast::functions::pretty_print_with_types_pretty_printer_alt_b::pretty_print_with_types_ast_stat_block;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parser::Parser;

    let code = r#"
        local s:string='str'
        local t:{a:string,b:number,[string]:number}
        local fn:(string,string)->(number,number)
        local s2:typeof(s)='foo'
        local os:string?
        local sn:string|number
        local it:{x:number}&{y:number}
    "#;
    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);

    let mut parse_result = Parser::parse(
        code,
        code.len(),
        &mut names,
        &mut allocator,
        ParseOptions::default(),
    );

    assert!(parse_result.errors.is_empty());
    assert!(!parse_result.root.is_null());
    let actual = unsafe { pretty_print_with_types_ast_stat_block(&mut *parse_result.root) };
    assert_eq!(code, actual);
}
