//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_recursive_type_alias_bad_pack_use_warns() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::generic_error::GenericError;
    use luaur_analysis::records::occurs_check_failed::OccursCheckFailed;
    use luaur_analysis::records::swapped_generic_type_parameter::SwappedGenericTypeParameter;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
type Foo<T> = Foo<T...>
"#,
        ),
        None,
    );

    assert_eq!(5, result.errors.len(), "{:?}", result.errors);
    assert!(
        result
            .errors
            .iter()
            .any(|error| type_error_data_ref::<GenericError>(error).is_some()),
        "expected GenericError in {:?}",
        result.errors
    );
    assert_eq!(
        "Generic type 'Foo<T>' expects 1 type argument, but none are specified",
        to_string_type_error(&result.errors[4])
    );
    type_error_data_ref::<OccursCheckFailed>(&result.errors[1])
        .expect("expected OccursCheckFailed");
    let swapped = type_error_data_ref::<SwappedGenericTypeParameter>(&result.errors[2])
        .expect("expected SwappedGenericTypeParameter");
    assert_eq!("T", swapped.name);
}
