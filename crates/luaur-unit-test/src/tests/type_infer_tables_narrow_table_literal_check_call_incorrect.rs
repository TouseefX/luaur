#[cfg(test)]
#[test]
fn type_infer_tables_narrow_table_literal_check_call_incorrect() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function take(_: { foo: string?, bing: number }) end

        take({ foo = "bar", bing = true })
    "#,
        ),
        None,
    );

    assert_eq!(1, results.errors.len(), "{:?}", results.errors);

    let err =
        type_error_data_ref::<TypeMismatch>(&results.errors[0]).expect("expected TypeMismatch");
    assert_eq!(
        Location::new(Position::new(3, 35), Position::new(3, 39)),
        results.errors[0].location
    );
    assert_eq!("boolean", to_string_type_id(err.given_type));
    assert_eq!("number", to_string_type_id(err.wanted_type));
}
