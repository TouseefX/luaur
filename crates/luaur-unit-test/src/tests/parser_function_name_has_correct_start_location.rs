#[cfg(test)]
#[test]
fn parser_function_name_has_correct_start_location() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_ast::records::position::Position;

    let mut fix = Fixture::default();
    let code = alloc::string::String::from(
        "\n        function simple()\n        end\n\n        function T:complex()\n        end\n    ",
    );
    let opts = ParseOptions::parse_options();
    let block = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_block::AstStatBlock>(
            fix.parse(&code, &opts) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    let body = unsafe { (*block).body };
    let size = body.size as usize;
    assert_eq!(size, 2);

    let first_stat = unsafe { *body.data.add(0) };
    let function1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_function::AstStatFunction>(
            first_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!function1.is_null());
    let name_loc = unsafe { (*function1).name };
    let name_loc = unsafe { (*name_loc).base.location };
    assert_eq!(name_loc.begin, Position::new(1, 17));

    let second_stat = unsafe { *body.data.add(1) };
    let function2 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_function::AstStatFunction>(
            second_stat as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!function2.is_null());
    let name_loc2 = unsafe { (*function2).name };
    let name_loc2 = unsafe { (*name_loc2).base.location };
    assert_eq!(name_loc2.begin, Position::new(4, 17));
}
