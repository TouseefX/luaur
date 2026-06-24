#[cfg(test)]
#[test]
fn parser_parse_global_declaration_called_class() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::ast_stat_declare_global::AstStatDeclareGlobal;
    use luaur_ast::records::ast_type_table::AstTypeTable;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FFlag::LuauAllowGlobalDeclarationToBeCalledClass;

    let _scoped_flag = LuauAllowGlobalDeclarationToBeCalledClass.set(true);

    let mut fixture = Fixture::default();
    let result = fixture.parse_ex(
        &alloc::string::String::from("declare class: { x: number }"),
        &ParseOptions::default(),
    );
    let root = unsafe { &*result.root };

    assert!(root.body.size > 0);
    assert_eq!(root.body.size, 1);

    let global = unsafe { &*root.body.data.add(0) };
    assert!(luaur_ast::rtti::ast_node_is::<AstStatDeclareGlobal>(
        *global as *mut luaur_ast::records::ast_node::AstNode
    ));

    // `global` is `&(*mut AstStat)` (a ref to the slot); RTTI-cast `*global`, not the
    // ref's own address (which made ast_node_as return null -> null deref on `&*`).
    let global = unsafe {
        &*luaur_ast::rtti::ast_node_as::<AstStatDeclareGlobal>(
            *global as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    // C++ `CHECK(global->name == "class")` — the global IS named "class"; the port
    // wrongly asserted a null/default name.
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr(global.name.value) },
        c"class"
    );

    assert!(!global.type_.is_null());
    assert!(luaur_ast::rtti::ast_node_is::<AstTypeTable>(unsafe {
        &*global.type_
    }));
}
