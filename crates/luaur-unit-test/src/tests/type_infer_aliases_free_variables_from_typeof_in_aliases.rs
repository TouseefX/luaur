//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_free_variables_from_typeof_in_aliases() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x) return x[1] end
        -- x has type X? for a free type variable X
        local x = f ({})
        type ContainsFree<a> = { this: a, that: typeof(x) }
        type ContainsContainsFree = { that: ContainsFree<number> }
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
