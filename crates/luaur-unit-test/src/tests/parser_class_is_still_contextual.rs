#[cfg(test)]
#[test]
fn parser_class_is_still_contextual() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::DebugLuauUserDefinedClasses;

    let _scoped_flag = ScopedFastFlag::new(&DebugLuauUserDefinedClasses, true);

    let mut fixture = Fixture::default();
    // The C++ R"(...)" DELIMITER parens leaked into the raw-string content here
    // (stray leading '(' / trailing ')' = invalid Lua). Drop them.
    let source = "
        local class = 42
        print(class)
    ";

    let result = fixture.try_parse(
        &source.to_string(),
        &luaur_ast::records::parse_options::ParseOptions::default(),
    );

    assert!(result.errors.is_empty());
    assert_eq!(unsafe { (*result.root).body.size }, 2);
    let locals = unsafe {
        luaur_ast::rtti::ast_node_as::<luaur_ast::records::ast_stat_local::AstStatLocal>(
            *(*result.root).body.data.add(0) as *mut luaur_ast::records::ast_node::AstNode,
        )
    };
    assert!(!locals.is_null());
    assert_eq!(unsafe { (*locals).vars.size }, 1);
    let var_name = unsafe { (*locals).vars.data.add(0).read() };
    assert_eq!(
        unsafe { core::ffi::CStr::from_ptr((*var_name).name.value) },
        c"class"
    );
}
