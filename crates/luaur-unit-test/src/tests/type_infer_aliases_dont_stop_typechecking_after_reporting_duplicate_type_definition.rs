//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_dont_stop_typechecking_after_reporting_duplicate_type_definition() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = number
        type A = string -- Redefinition of type 'A', previously defined at line 1
        local foo: string = 1 -- "Type 'number' could not be converted into 'string'"
    "#,
        ),
        None,
    );

    assert_eq!(2, result.errors.len(), "{:?}", result.errors);
}
