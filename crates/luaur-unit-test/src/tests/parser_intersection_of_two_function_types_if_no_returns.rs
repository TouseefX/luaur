#[cfg(test)]
#[test]
fn parser_intersection_of_two_function_types_if_no_returns() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;

    let mut fixture = Fixture::default();
    let block = fixture.parse(
        "local f: (string) -> () & (number) -> ()",
        &ParseOptions::default(),
    );
    assert!(!block.is_null());

    let local = unsafe { (*block).body.data.add(0) };
    let local = unsafe { &*local };
    let local = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *local as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!local.is_null());

    // C++ `local->vars.data[0]->annotation->as<AstTypeIntersection>()` — deref the
    // AstLocal slot, then take its `.annotation` type, then RTTI-cast.
    let var0 = unsafe { *(*local).vars.data.add(0) };
    let annotation = unsafe { (*var0).annotation };
    let annotation = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_intersection::AstTypeIntersection>(
            annotation as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!annotation.is_null());

    let annotation = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_intersection::AstTypeIntersection>(
            annotation as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!annotation.is_null());

    // C++ `annotation->types.data[i]->as<AstTypeFunction>()` — deref the slot to the
    // AstType element, then RTTI-cast (was casting the slot address + double-casting).
    let ty0 = unsafe { *(*annotation).types.data.add(0) };
    let ty0 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            ty0 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ty0.is_null());

    let ty1 = unsafe { *(*annotation).types.data.add(1) };
    let ty1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            ty1 as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ty1.is_null());
    let ty1 = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_type_function::AstTypeFunction>(
            ty1 as *const _ as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!ty1.is_null());
}
