//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_mutually_recursive_types_restriction_not_ok_1() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        -- OK because forwarded types are used with their parameters.
        type Tree<T> = { data: T, children: Forest<T> }
        type Forest<T> = {Tree<{T}>}
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
