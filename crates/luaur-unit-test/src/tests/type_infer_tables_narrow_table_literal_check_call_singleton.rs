#[cfg(test)]
#[test]
fn type_infer_tables_narrow_table_literal_check_call_singleton() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);

    let mut fixture = Fixture::fixture_bool(false);
    let results = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function take(_: { foo: "foo" }) end

        take({ foo = "foo" })
    "#,
        ),
        None,
    );

    assert_eq!(0, results.errors.len(), "{:?}", results.errors);
}
