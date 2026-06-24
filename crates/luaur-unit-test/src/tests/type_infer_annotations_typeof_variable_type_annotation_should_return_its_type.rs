//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_typeof_variable_type_annotation_should_return_its_type() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local foo = { bar = "baz" }

        type Foo = typeof(foo)

        local foo2: Foo
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        fixture.require_type_string(&String::from("foo")),
        fixture.require_type_string(&String::from("foo2"))
    );
}
