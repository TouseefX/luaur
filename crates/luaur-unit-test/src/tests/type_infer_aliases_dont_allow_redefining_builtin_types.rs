//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_dont_allow_redefining_builtin_types() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::duplicate_type_definition::DuplicateTypeDefinition;
    use luaur_common::FFlag;

    let _disallow_redefining =
        ScopedFastFlag::new(&FFlag::LuauDisallowRedefiningBuiltinTypes, true);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();
    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type number = string
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<DuplicateTypeDefinition>(&result.errors[0])
        .expect("expected DuplicateTypeDefinition");
}
