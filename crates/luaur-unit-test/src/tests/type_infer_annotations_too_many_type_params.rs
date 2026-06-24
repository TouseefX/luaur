//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_too_many_type_params() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::records::incorrect_generic_parameter_count::IncorrectGenericParameterCount;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type Callback<A, R> = (A) -> (boolean, R)
        local a: Callback<number, number, string> = function(i) return true, 4 end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(2, result.errors[0].location.begin.line);

    let igpc = type_error_data_ref::<IncorrectGenericParameterCount>(&result.errors[0])
        .expect("expected IncorrectGenericParameterCount");
    assert_eq!(3, igpc.actual_parameters());
    assert_eq!(2, igpc.type_fun().type_params().len());
    assert_eq!("Callback", igpc.name());

    assert_eq!(
        "Generic type 'Callback<A, R>' expects 2 type arguments, but 3 are specified",
        to_string_type_error(&result.errors[0])
    );
}
