//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_intersection_of_tables() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f(x: { p : number?, q : string? } & { p : number?, q : number?, r : number? })
            local y : { p : number?, q : nil, r : number? } = x -- OK
            local z : { p : nil } = x -- Not OK
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        concat!(
            "Expected this to be '{ p: nil }', but got '{ p: number?, q: number?, r: number? } & { p: number?, q: string? }",
            "'; \nthis is because \n\t",
            " * in the 1st component of the intersection, accessing `p` has the 1st component of the union as `number` and ",
            "accessing `p` results in `nil`, and `number` is not exactly `nil`\n\t",
            " * in the 2nd component of the intersection, accessing `p` has the 1st component of the union as `number` and ",
            "accessing `p` results in `nil`, and `number` is not exactly `nil`"
        )
    } else {
        "Expected this to be '{ p: nil }', but got '{ p: number?, q: number?, r: number? } & { p: number?, q: string? }'; none of the intersection parts are compatible"
    };

    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
