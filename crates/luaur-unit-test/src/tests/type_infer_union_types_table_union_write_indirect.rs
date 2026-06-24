//! Ported from `tests/TypeInfer.unionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_union_types_table_union_write_indirect() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type A = { x: number, y: (number) -> string } | { z: number, y: (number) -> string }

        function f(a: A)
            function a.y(x)
                return tostring(x * 2)
            end

            function a.y(x: string): number
                return tonumber(x) or 0
            end
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected = "Expected this to be\n\t\
'((number) -> string) | ((number) -> string)'\
\nbut got\n\t\
'(string) -> number'\
; none of the union options are compatible";
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
}
