//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_forward_declared_alias_is_not_clobbered_by_prior_unification_with_any() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function x()
            local y: FutureType = {}::any
            return 1
        end
        type FutureType = { foo: typeof(x()) }
        local d: FutureType = { smth = true } -- missing error, 'd' is resolved to 'any'
    "#,
        ),
        None,
    );

    let mut opts = ToStringOptions::to_string_options(true);
    assert_eq!(
        "{ foo: number }",
        to_string_type_id_to_string_options(
            fixture.require_type_string(&String::from("d")),
            &mut opts
        )
    );
    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
}
