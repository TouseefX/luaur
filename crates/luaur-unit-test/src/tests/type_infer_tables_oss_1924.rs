#[cfg(test)]
#[test]
fn type_infer_tables_oss_1924() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t: { [string]: "s" } = {
            key = "s",
            other_key = "t",
        }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err =
        type_error_data_ref::<TypeMismatch>(&result.errors[0]).expect("expected TypeMismatch");
    assert_eq!("\"s\"", to_string_type_id(err.wanted_type));
    assert_eq!("\"t\"", to_string_type_id(err.given_type));
}
