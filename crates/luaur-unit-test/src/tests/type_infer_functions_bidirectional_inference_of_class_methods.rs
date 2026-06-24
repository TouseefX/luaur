#[cfg(test)]
#[test]
fn type_infer_functions_bidirectional_inference_of_class_methods() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::unknown_property::UnknownProperty;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local c = ChildClass.New()

        -- Instead of reporting that the lambda is the wrong type, report that we are using its argument improperly.
        c.Touched:Connect(function(other)
            print(other.ThisDoesNotExist)
        end)
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let err = type_error_data_ref::<UnknownProperty>(&result.errors[0])
        .expect("expected UnknownProperty");
    assert_eq!("ThisDoesNotExist", err.key());
    assert_eq!("BaseClass", to_string_type_id(err.table()));
}
