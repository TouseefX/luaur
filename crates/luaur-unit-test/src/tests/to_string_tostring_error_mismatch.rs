//! Ported from `tests/ToString.test.cpp`.

#[cfg(test)]
#[test]
fn to_string_tostring_error_mismatch() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        --!strict
        function f1(t: {a : number, b: string, c: {d: string}}) : {a : number, b : string, c : { d : number}}
            return t
        end
    "#,
        ),
        None,
    );

    let expected = if !FFlag::DebugLuauForceOldSolver.get() {
        "Expected this to be\n\t'{ a: number, b: string, c: { d: number } }'\nbut got\n\t'{ a: number, b: string, c: { d: string } }'; \naccessing `c.d` results in `string` in the latter type and `number` in the former type, and `string` is not exactly `number`"
    } else {
        "Expected this to be exactly\n\t'{ a: number, b: string, c: { d: number } }'\nbut got\n\t'{ a: number, b: string, c: { d: string } }'\ncaused by:\n  Property 'c' is not compatible.\nExpected this to be exactly\n\t'{ d: number }'\nbut got\n\t'{ d: string }'\ncaused by:\n  Property 'd' is not compatible.\nExpected this to be exactly 'number', but got 'string'"
    };

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
