//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_generic_param_remap() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);
    let code = String::from(
        r#"
        -- An example of a forwarded use of a type that has different type arguments than parameters
        type A<T,U> = {t:T, u:U, next:A<U,T>?}
        local aa:A<number,string> = { t = 5, u = 'hi', next = { t = 'lo', u = 8 } }
        local bb = aa
    "#,
    );
    let expected = String::from(
        r#"

        type A<T,U> = {t:T, u:U, next:A<U,T>?}
        local aa:A<number,string> = { t = 5, u = 'hi', next = { t = 'lo', u = 8 } }
        local bb:A<number,string>=aa
    "#,
    );

    assert_eq!(expected, fixture.decorate_with_types(&code));
    let result = fixture.check_string_optional_frontend_options(&code, None);
    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
