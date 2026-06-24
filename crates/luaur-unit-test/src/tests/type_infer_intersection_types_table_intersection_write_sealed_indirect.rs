//! Ported from `tests/TypeInfer.intersectionTypes.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_intersection_types_table_intersection_write_sealed_indirect() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::LuauCheckFunctionStatementTypes, true);

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type X = { x: (number) -> number }
        type Y = { y: (string) -> string }

        type XY = X & Y

        function f(t: XY)
            function t.z(a:number) return a * 10 end
            function t:y(a:number) return a * 10 end
            function t:w(a:number) return a * 10 end
        end
    "#,
        ),
        None,
    );

    assert_eq!(4, result.errors.len(), "{:?}", result.errors);
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "Cannot add property 'z' to table 'X & Y'",
            to_string_type_error(&result.errors[0])
        );
        let err1 = unsafe { get_type_error::<TypeMismatch>(&result.errors[1]).as_ref() }
            .expect("expected TypeMismatch");
        assert_eq!("number", to_string_type_id(err1.given_type));
        assert_eq!("string", to_string_type_id(err1.wanted_type));
        let err2 = unsafe { get_type_error::<TypeMismatch>(&result.errors[2]).as_ref() }
            .expect("expected TypeMismatch");
        assert_eq!(
            "(string, number) -> string",
            to_string_type_id(err2.given_type)
        );
        assert_eq!("(string) -> string", to_string_type_id(err2.wanted_type));
        assert_eq!(
            "Cannot add property 'w' to table 'X & Y'",
            to_string_type_error(&result.errors[3])
        );
    } else {
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
            "Cannot add property 'z' to table 'X & Y'",
            to_string_type_error(&result.errors[1])
        );
        assert_eq!(
            "Expected this to be 'string', but got 'number'",
            to_string_type_error(&result.errors[2])
        );
        assert_eq!(
            "Cannot add property 'w' to table 'X & Y'",
            to_string_type_error(&result.errors[3])
        );
    }
}
