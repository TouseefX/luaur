#[cfg(test)]
#[test]
fn type_infer_tables_cli_119126_regression() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type literals = "foo" | "bar" | "foobar"

        local exampleA: {[literals]: string} = {
            foo = '1',
            bar = 2,
            foobar = 3,
        }
    "#,
        ),
        None,
    );

    assert_eq!(2, results.errors.len(), "{:?}", results.errors);
    for err in &results.errors {
        let mismatch = type_error_data_ref::<TypeMismatch>(err).expect("expected TypeMismatch");
        assert_eq!("string", to_string_type_id(mismatch.wanted_type));
        assert_eq!("number", to_string_type_id(mismatch.given_type));
    }
}
