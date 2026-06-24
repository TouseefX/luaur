//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_recursive_types_restriction_not_ok() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, true);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        -- this would be an infinite type if we allowed it
        type Tree<T> = { data: T, children: {Tree<{T}>} }
    "#,
        ),
        None,
    );

    assert!(!result.errors.is_empty(), "{:?}", result.errors);
}
