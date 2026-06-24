//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_type_alias_dont_crash_on_duplicate_with_typeof() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = typeof(nil :: any)
        type A = typeof(nil :: any)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
        .expect("expected DuplicateTypeDefinition");
}
