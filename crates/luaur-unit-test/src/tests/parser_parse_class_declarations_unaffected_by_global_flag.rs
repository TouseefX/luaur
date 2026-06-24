#[cfg(test)]
#[test]
fn parser_parse_class_declarations_unaffected_by_global_flag() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FFlag::LuauAllowGlobalDeclarationToBeCalledClass;

    let _flag = ScopedFastFlag::new(&LuauAllowGlobalDeclarationToBeCalledClass, true);
    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
        &alloc::string::String::from(
            "declare class Foo\n\
             \tprop: number\n\
             end",
        ),
        &ParseOptions::default(),
    );

    let root = unsafe { &*result.root };
    assert!(!root.body.data.is_null());
    assert_eq!(root.body.size, 1);

    let declared = unsafe {
        luaur_ast::rtti::ast_node_as::<
            luaur_ast::records::ast_stat_declare_extern_type::AstStatDeclareExternType,
        >(*root.body.data as *mut luaur_ast::records::ast_node::AstNode)
    };
    assert!(!declared.is_null());
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*declared).name.value) },
        c"Foo"
    );
}
