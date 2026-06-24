#[cfg(test)]
#[test]
fn parser_classes_work_after_other_statements() {
    use crate::records::fixture::Fixture;
    use luaur_ast::records::parse_options::ParseOptions;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _scoped_flag = luaur_common::FFlag::DebugLuauUserDefinedClasses.set(true);

    let mut fixture = Fixture::default();
    let source = r#"
        if math.random() > 0.5 then
            print("I am a test case!")
        end

        class Player
            public health: number
        end
    "#;

    let result = fixture.try_parse(&source.to_string(), &ParseOptions::parse_options());

    assert_eq!(result.errors.len(), 0);

    assert_eq!(2, unsafe { (*result.root).body.size });
    let cls = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_class::AstStatClass>(
            *(*result.root).body.data.add(1) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(cls.is_null() == false);
    // C++ `CHECK_EQ(cls->name->name, "Player")` compares AstName content (strcmp),
    // not the interned pointer. The port compared `name.value` against the literal's
    // pointer, which never matches. Compare NUL-terminated content instead.
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*cls).name.as_ref().unwrap().name.value) },
        c"Player"
    );
}
