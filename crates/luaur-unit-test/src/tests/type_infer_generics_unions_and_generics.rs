#[cfg(test)]
#[test]
fn type_infer_generics_unions_and_generics() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type foo = <T>(T | {T}) -> T
        local foo = (nil :: any) :: foo

        type Test = number | {number}
        local res = foo(1 :: Test)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "number",
            to_string_type_id(fixture.require_type_string(&String::from("res")))
        );
    } else {
        assert_eq!(
            "'a",
            to_string_type_id(fixture.require_type_string(&String::from("res")))
        );
    }
}
