//! Ported from `tests/RuntimeLimits.test.cpp`.

#[cfg(test)]
#[test]
fn runtime_limits_comparison_to_nil_when_normalization_fails_should_not_crash() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::String;
    use luaur_common::{FFlag, FInt};

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let _fuel = ScopedFastInt::new(&FInt::LuauNormalizerInitialFuel, 3);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = { foo: number } | { bar: number } | { baz: number }
        type U = { oof: number } | { rab: number } | { zab: number }
        type TU = T & U
        local function check(t: TU): boolean
            return t == nil
        end
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
