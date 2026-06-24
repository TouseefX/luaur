#[cfg(test)]
#[test]
fn type_infer_generics_check_mutual_generic_functions_unannotated() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function id1(x)
            local y: string = id2("hi")
            local z: number = id2(37)
            return x
        end

        function id2(x)
            local y: string = id1("hi")
            local z: number = id1(37)
            return x
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
