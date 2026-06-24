//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_pick_distinct_names_for_mixed_explicit_and_implicit_generics() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function foo<a>(x: a, y) end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "<a>(a, unknown) -> ()",
            to_string_type_id(fixture.require_type_string(&String::from("foo")))
        );
    } else {
        assert_eq!(
            "<a, b>(a, b) -> ()",
            to_string_type_id(fixture.require_type_string(&String::from("foo")))
        );
    }
}
