#[cfg(test)]
#[test]
fn parser_export_class() {
    use crate::records::fixture::Fixture;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _scoped_flag = luaur_common::FFlag::DebugLuauUserDefinedClasses.set(true);

    let mut fixture = Fixture::default();
    let source = "\n        export class Foo\n        end\n    ";

    let result = fixture.try_parse(
        &alloc::string::String::from(source),
        &luaur_ast::records::parse_options::ParseOptions::parse_options(),
    );

    assert_eq!(result.errors.len(), 0);

    let block = unsafe { &*result.root };
    assert_eq!(block.body.size, 1);

    let stat_ptr = unsafe { *block.body.data.add(0) };
    let class_decl = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_class::AstStatClass>(
            stat_ptr as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!class_decl.is_null());
    let class_decl = unsafe { &*class_decl };
    assert!(class_decl.exported);
}
