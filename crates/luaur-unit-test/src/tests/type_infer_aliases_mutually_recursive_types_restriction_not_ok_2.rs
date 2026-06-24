//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_mutually_recursive_types_restriction_not_ok_2() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        -- Not OK because forwarded types are used with different types than their parameters.
        type Forest<T> = {Tree<{T}>}
        type Tree<T> = { data: T, children: Forest<T> }
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
