//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_incomplete_function_annotation() {
    use crate::records::fixture::Fixture;
    use luaur_ast::enums::mode::Mode;

    let mut fixture = Fixture::default();

    let result = fixture.check_string_optional_frontend_options(
        &alloc::string::String::from(
            r#"
        local x: () ->
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty());
}
