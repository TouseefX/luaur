//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_table_write_sealed_indirect() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
    type XY = { x: (number) -> number, y: (string) -> string }

    local xy : XY = {
        x = function(a: number) return -a end,
        y = function(a: string) return a .. "b" end
    }
    function xy.z(a:number) return a * 10 end
    function xy:y(a:number) return a * 10 end
    function xy:w(a:number) return a * 10 end
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    let expected = concat!(
        "Expected this to be\n\t",
        "'(string) -> string'",
        "\nbut got\n\t",
        "'(string, number) -> string'",
        "\ncaused by:\n",
        "  Argument count mismatch. Function expects 2 arguments, but only 1 is specified"
    );
    assert_eq!(expected, to_string_type_error(&result.errors[0]));
    assert_eq!(
        "Cannot add property 'z' to table 'XY'",
        to_string_type_error(&result.errors[1])
    );
    assert_eq!(
        "Expected this to be 'string', but got 'number'",
        to_string_type_error(&result.errors[2])
    );
    assert_eq!(
        "Cannot add property 'w' to table 'XY'",
        to_string_type_error(&result.errors[3])
    );
}
