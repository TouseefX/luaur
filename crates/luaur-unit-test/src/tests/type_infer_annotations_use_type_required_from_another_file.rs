//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_use_type_required_from_another_file() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::add_global_binding_builtin_definitions::add_global_binding_builtin_definitions;

    let mut fixture = BuiltinsFixture::default();
    let any_type = {
        let frontend = fixture.get_frontend();
        unsafe { (*frontend.builtin_types).anyType }
    };
    add_global_binding_builtin_definitions(
        &mut fixture.get_frontend().globals,
        "script",
        any_type,
        "@test",
    );

    fixture.base.file_resolver.source.insert(
        String::from("Modules/Main"),
        String::from(
            r#"
        --!strict
        local Test = require(script.Parent.Thing)

        export type Foo = { [any]: Test.TestType }

        return Test
    "#,
        ),
    );

    fixture.base.file_resolver.source.insert(
        String::from("Modules/Thing"),
        String::from(
            r#"
        --!strict

        export type TestType = {bar: boolean}

        return {}
    "#,
        ),
    );

    let result = fixture
        .get_frontend()
        .check_module_name_optional_frontend_options(&String::from("Modules/Main"), None);

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
