//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_warn_if_you_try_to_require_a_non_modulescript() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::enums::type_file_resolver::Type as SourceCodeType;
    use luaur_analysis::records::illegal_require::IllegalRequire;

    let mut fixture = BuiltinsFixture::default();

    fixture
        .base
        .file_resolver
        .source
        .insert(String::from("Modules/A"), String::new());
    fixture
        .base
        .file_resolver
        .source_types
        .insert(String::from("Modules/A"), SourceCodeType::Script);
    fixture.base.file_resolver.source.insert(
        String::from("Modules/B"),
        String::from(
            r#"
        local M = require(script.Parent.A)
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/B"), None);

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert!(type_error_data_ref::<IllegalRequire>(&result.errors[0]).is_some());
}
