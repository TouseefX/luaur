#[cfg(test)]
#[test]
fn parser_parse_type_name() {
    use alloc::string::String;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::ast_type_function::AstTypeFunction;
    use luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit;
    use luaur_ast::records::parse_node_result::ParseNodeResult;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parser::Parser;

    let code = String::from("<A>(A, string, boolean?) -> number");
    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);

    let result: ParseNodeResult<luaur_ast::records::ast_type::AstType> =
        Parser::parse_type_c_char_usize_ast_name_table_allocator_parse_options(
            &code,
            &mut names,
            &mut allocator,
            ParseOptions::parse_options(),
        );

    assert!(result.errors.is_empty());
    assert!(!result.root.is_null());

    let fun = unsafe { (*result.root).base.as_item::<AstTypeFunction>() };
    assert!(!fun.is_null());

    let generics = unsafe { (*fun).generics };
    assert_eq!(1, generics.size as usize);

    let arg_types = unsafe { (*fun).arg_types };
    assert_eq!(3, arg_types.types.size as usize);

    let return_types = unsafe { (*fun).return_types };
    let return_pack = unsafe { (*return_types).base.as_item::<AstTypePackExplicit>() };
    assert!(!return_pack.is_null());

    let type_list = unsafe { (*return_pack).type_list };
    assert_eq!(1, type_list.types.size as usize);
}
