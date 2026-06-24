//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_index_on_an_intersection_type_with_property_guaranteed_to_exist() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = {x: {y: number}}
        type B = {x: {y: number}}

        local function f(t: A & B)
            return t.x
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "(A & B) -> { y: number }"
    } else {
        "(A & B) -> { y: number } & { y: number }"
    };
    assert_eq!(
        expected,
        to_string_type_id(fixture.require_type_string(&String::from("f")))
    );
}
