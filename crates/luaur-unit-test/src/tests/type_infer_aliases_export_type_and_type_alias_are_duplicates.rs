//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_export_type_and_type_alias_are_duplicates() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type Foo = number
        type Foo = number
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let dtd = type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
        .expect("expected DuplicateTypeDefinition");
    assert_eq!("Foo", dtd.name());
}
