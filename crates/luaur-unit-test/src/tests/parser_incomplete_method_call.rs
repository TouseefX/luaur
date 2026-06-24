#[cfg(test)]
#[test]
fn parser_incomplete_method_call() {
    use luaur_analysis::records::source_module::SourceModule;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::ast_stat_return::AstStatReturn;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::parser::Parser;

    let source = "        function howdy()
            return game:
        end
    ";
    let mut source_module = SourceModule::source_module();
    let names = alloc::sync::Arc::get_mut(&mut source_module.names).unwrap();
    let allocator = alloc::sync::Arc::get_mut(&mut source_module.allocator).unwrap();
    let options = ParseOptions::parse_options();
    let result: ParseResult = Parser::parse(source, source.len(), names, allocator, options);

    assert_eq!(1, result.root.is_null() as i32 ^ 1);
    assert_eq!(1, unsafe { (*result.root).body.size });

    let howdy_function = unsafe { (*result.root).body.data.add(0).read() };
    let howdy_function = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatFunction>(
            howdy_function as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!howdy_function.is_null());

    let body = unsafe { (*howdy_function).func.read().body };
    assert_eq!(1, unsafe { (*body).body.size });

    let ret_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatReturn>(
            (*body).body.data.add(0).read() as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!ret_stat.is_null());

    let func_loc = unsafe { (*howdy_function).base.base.location };
    let body_loc = unsafe { (*body).base.base.location };
    assert!(func_loc.end > body_loc.end);
}
