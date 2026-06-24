//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_overloadeded_functions_with_weird_typepacks_1() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        function f<a...,b...>()
            function g(x : (() -> a...) & (() -> b...))
                local y : (() -> b...) & (() -> a...) = x -- OK
                local z : () -> () = x -- Not OK
            end
        end
    "#,
        ),
        None,
    );

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    } else {
        assert_eq!(1, result.errors.len(), "{:?}", result.errors);
        assert_eq!(
            "Expected this to be '() -> ()', but got '(() -> (a...)) & (() -> (b...))'; none of the intersection parts are compatible",
            to_string_type_error(&result.errors[0])
        );
    }
}
