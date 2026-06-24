#[cfg(test)]
#[test]
fn pretty_printer_pretty_print_to_string() {
    use crate::records::fixture::Fixture;
    use luaur_ast::functions::to_string_pretty_printer::to_string_ast_node;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::ast_node::AstNode;
    use luaur_ast::records::ast_stat_local::AstStatLocal;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parser::Parser;

    let _fixture = Fixture::default();
    let code = "local a: string = 'hello'";

    let mut allocator = Allocator::allocator();
    let mut names = AstNameTable::new(&mut allocator);
    let parse_result = Parser::parse(
        code,
        code.len(),
        &mut names,
        &mut allocator,
        ParseOptions::default(),
    );

    assert!(!parse_result.root.is_null());
    let root = unsafe { &*parse_result.root };
    assert_eq!(1, root.body.size);

    let stat = unsafe { *root.body.data.add(0) };
    let stat_local = unsafe { luaur_ast::rtti::ast_node_as::<AstStatLocal>(stat as *mut AstNode) };
    assert!(!stat_local.is_null());
    assert_eq!(
        "local a: string = 'hello'",
        to_string_ast_node(stat_local as *mut AstNode)
    );

    let stat_local = unsafe { &*stat_local };
    assert_eq!(1, stat_local.vars.size);
    let local = unsafe { *stat_local.vars.data.add(0) };
    let annotation = unsafe { (*local).annotation };
    assert!(!annotation.is_null());
    assert_eq!("string", to_string_ast_node(annotation as *mut AstNode));

    assert_eq!(1, stat_local.values.size);
    let expr = unsafe { *stat_local.values.data.add(0) };
    assert_eq!("'hello'", to_string_ast_node(expr as *mut AstNode));
}
