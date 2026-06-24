#[cfg(test)]
#[test]
fn type_infer_generics_dont_leak_inferred_generic_types() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f(y)
            local z = y
            local function id(x)
                z = x
                return x
            end
            local x: string = id("hi")
            local y: number = id(37)
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        assert!(!result.errors.is_empty(), "{:?}", result.errors);
    }
}
