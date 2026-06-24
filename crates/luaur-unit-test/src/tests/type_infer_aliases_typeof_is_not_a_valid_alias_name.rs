//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_typeof_is_not_a_valid_alias_name() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type typeof = number
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "typeof cannot be used as an identifier for a type function or alias"
    } else {
        "Type aliases cannot be named typeof"
    };
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
