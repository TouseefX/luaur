#[cfg(test)]
#[test]
fn type_infer_generics_better_mismatch_error_messages() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::swapped_generic_type_parameter::SwappedGenericTypeParameter;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<T>(...: T...)
            return ...
        end

        function g<T...>(a: T)
            return a
        end
    "#,
        ),
        None,
    );

    let (f_err_index, g_err_index) = if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(3, result.errors.len(), "{:?}", result.errors);
        (1, 2)
    } else {
        assert_eq!(2, result.errors.len(), "{:?}", result.errors);
        (0, 1)
    };

    let f_err = type_error_data_ref::<SwappedGenericTypeParameter>(&result.errors[f_err_index])
        .expect("expected SwappedGenericTypeParameter");
    assert_eq!("T", f_err.name);
    assert_eq!(SwappedGenericTypeParameter::Pack, f_err.kind);

    let g_err = type_error_data_ref::<SwappedGenericTypeParameter>(&result.errors[g_err_index])
        .expect("expected SwappedGenericTypeParameter");
    assert_eq!("T", g_err.name);
    assert_eq!(SwappedGenericTypeParameter::Type, g_err.kind);
}
