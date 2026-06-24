//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_require_module_that_does_not_export() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::illegal_require::IllegalRequire;

    let mut fixture = BuiltinsFixture::default();

    fixture
        .base
        .file_resolver
        .source
        .insert(String::from("game/Workspace/A"), String::from(""));
    fixture.base.file_resolver.source.insert(
        String::from("game/Workspace/B"),
        String::from(
            r#"
        local Hooty = require(script.Parent.A)
    "#,
        ),
    );

    let _ = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Workspace/A"), None);
    let _ = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("game/Workspace/B"), None);

    let a_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/Workspace/A"));
    let b_module = fixture
        .get_frontend()
        .module_resolver
        .get_module(&String::from("game/Workspace/B"));

    assert!(a_module.errors.is_empty(), "{:?}", a_module.errors);
    assert_eq!(1, b_module.errors.len(), "{:?}", b_module.errors);
    assert!(
        type_error_data_ref::<IllegalRequire>(&b_module.errors[0]).is_some(),
        "Should be IllegalRequire: {:?}",
        b_module.errors[0]
    );

    let hooty_type = fixture
        .base
        .require_type_module_ptr_string(&b_module, &String::from("Hooty"));
    assert_eq!("*error-type*", to_string_type_id(hooty_type));
}
