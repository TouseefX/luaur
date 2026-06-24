#[cfg(test)]
#[test]
fn parser_incomplete_method_call_2() {
    use luaur_analysis::records::source_module::SourceModule;
    use luaur_ast::records::allocator::Allocator;
    use luaur_ast::records::ast_name_table::AstNameTable;
    use luaur_ast::records::ast_stat_block::AstStatBlock;
    use luaur_ast::records::ast_stat_error::AstStatError;
    use luaur_ast::records::ast_stat_function::AstStatFunction;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::parse_result::ParseResult;
    use luaur_ast::records::parser::Parser;

    let source = alloc::string::String::from(
        "local game = { GetService=function(s) return 'hello' end }\n\n\
         function a()\n\
             game:a\n\
         end",
    );

    let mut source_module = SourceModule::source_module();
    let options = ParseOptions::parse_options();
    let result: ParseResult = Parser::parse(
        source.as_str(),
        source.len(),
        alloc::sync::Arc::get_mut(&mut source_module.names).unwrap(),
        alloc::sync::Arc::get_mut(&mut source_module.allocator).unwrap(),
        options,
    );

    assert_eq!(2, unsafe { (*result.root).body.size });

    let howdy_function: *mut AstStatFunction = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatFunction>(
            (*result.root).body.data.add(1).read() as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!howdy_function.is_null());

    let body: *mut AstStatBlock = unsafe { (*howdy_function).func.read().body };
    assert_eq!(1, unsafe { (*body).body.size });

    let ret: *mut AstStatError = unsafe {
        luaur_ast::rtti::ast_node_as::<AstStatError>(
            (*body).body.data.add(0).read() as *mut luaur_ast::records::ast_node::AstNode
        )
    };
    assert!(!ret.is_null());

    assert!(
        unsafe { (*howdy_function).base.base.location.end }
            > unsafe { (*body).base.base.location.end }
    );
}
