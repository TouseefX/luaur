#[cfg(test)]
#[test]
fn type_infer_generics_check_mutual_generic_functions_errors() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id1(x)
            local y: string = id2(37) -- odd
            local z: number = id2("hi") -- even
            return x
        end

        function id2(x)
            local y: string = id1(37) -- odd
            local z: number = id1("hi") -- even
            return x
        end
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);

    for i in (0..4).step_by(2) {
        let tm =
            type_error_data_ref::<TypeMismatch>(&result.errors[i]).expect("expected TypeMismatch");
        assert_eq!("string", to_string_type_id(tm.wanted_type));
        assert_eq!("number", to_string_type_id(tm.given_type));
    }

    for i in (1..4).step_by(2) {
        let tm =
            type_error_data_ref::<TypeMismatch>(&result.errors[i]).expect("expected TypeMismatch");
        assert_eq!("number", to_string_type_id(tm.wanted_type));
        assert_eq!("string", to_string_type_id(tm.given_type));
    }
}
