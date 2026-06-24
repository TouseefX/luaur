//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_should_still_pick_an_overload_whose_arguments_are_unions() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = (number) -> string
        type B = (string) -> number

        local function foo(f: A & B)
            return f(1), f("five")
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "(((number) -> string) & ((string) -> number)) -> (string, number)",
        to_string_type_id(fixture.require_type_string(&String::from("foo")))
    );
}
