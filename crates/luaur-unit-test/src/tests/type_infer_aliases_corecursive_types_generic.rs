//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_corecursive_types_generic() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from(
        r#"
        type A<T> = {v:T, b:B<T>}
        type B<T> = {v:T, a:A<T>}

        function f(a: A<number>)
            return a
        end
    "#,
    );
    let expected = String::from(
        r#"
        type A<T> = {v:T, b:B<T>}
        type B<T> = {v:T, a:A<T>}

        function f(a: A<number>): A<number>
            return a
        end
    "#,
    );

    assert_eq!(expected, fixture.decorate_with_types(&code));
    let result = fixture.check_string_optional_frontend_options(&code, None);
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
