//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_infer_type_of_value_a_via_typeof_with_assignment() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local a
        local b: typeof(a) = 1

        a = "foo"
    "#,
        ),
        None,
    );

    let expected_location;
    let expected_wanted;
    let expected_given;

    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "string?",
            to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
        assert_eq!(
            "nil",
            to_string_type_id(fixture.require_type_string(&String::from("b")))
        );

        expected_location = Location {
            begin: Position {
                line: 2,
                column: 29,
            },
            end: Position {
                line: 2,
                column: 30,
            },
        };
        expected_wanted = fixture.get_builtins().nilType;
        expected_given = fixture.get_builtins().numberType;
    } else {
        assert_eq!(
            "number",
            to_string_type_id(fixture.require_type_string(&String::from("a")))
        );
        assert_eq!(
            "number",
            to_string_type_id(fixture.require_type_string(&String::from("b")))
        );

        expected_location = Location {
            begin: Position {
                line: 4,
                column: 12,
            },
            end: Position {
                line: 4,
                column: 17,
            },
        };
        expected_wanted = fixture.get_builtins().numberType;
        expected_given = fixture.get_builtins().stringType;
    }

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(expected_location, result.errors[0].location);

    let tm = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!(expected_wanted, tm.wanted_type);
    assert_eq!(expected_given, tm.given_type);
}
