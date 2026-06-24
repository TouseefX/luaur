#[cfg(test)]
#[test]
fn parser_parse_variadics() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let stat = fixture.parse_ex(
        &alloc::string::String::from(
            "function foo(bar, ...: number): ...string\n\
             end\n\
             \n\
             type Foo = (string, number, ...number) -> ...boolean\n\
             type Bar = () -> (number, ...boolean)",
        ),
        &ParseOptions::default(),
    );
    let root = unsafe { &*stat.root };
    assert_eq!(3, root.body.size);

    let fn_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_function::AstStatFunction>(
            (*root.body.data.add(0)) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!fn_stat.is_null());
    let func = unsafe { &*(*fn_stat).func };
    assert!(func.vararg);
    assert!(!func.vararg_annotation.is_null());

    let foo_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
            (*root.body.data.add(1)) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!foo_stat.is_null());
    let foo_type = unsafe { &*(*foo_stat).type_ptr };
    let foo_fn = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            foo_type as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!foo_fn.is_null());
    let foo_fn = unsafe { &*foo_fn };
    assert_eq!(2, foo_fn.arg_types.types.size);
    assert!(!foo_fn.arg_types.tail_type.is_null());
    let return_tp = unsafe { &*foo_fn.return_types };
    assert!(luaur_ast::rtti::ast_node_is::<
        luaur_ast::records::ast_type_pack_variadic::AstTypePackVariadic,
    >(return_tp));

    let bar_stat = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_type_alias::AstStatTypeAlias>(
            (*root.body.data.add(2)) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!bar_stat.is_null());
    let bar_type = unsafe { &*(*bar_stat).type_ptr };
    let bar_fn = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            bar_type as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!bar_fn.is_null());
    let bar_fn = unsafe { &*bar_fn };
    assert_eq!(0, bar_fn.arg_types.types.size);
    assert!(bar_fn.arg_types.tail_type.is_null());
    let return_tp = unsafe { &*bar_fn.return_types };
    let explicit_pack = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_pack_explicit::AstTypePackExplicit>(
            return_tp as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!explicit_pack.is_null());
    let explicit_pack = unsafe { &*explicit_pack };
    assert_eq!(1, explicit_pack.type_list.types.size);
    assert!(!explicit_pack.type_list.tail_type.is_null());
}
