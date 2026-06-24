//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_cannot_steal_hoisted_type_alias() {
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
        local x: T = "foo"
        type T = number
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let expected_location = if !FFlag::DebugLuauForceOldSolver.get() {
        Location {
            begin: Position {
                line: 1,
                column: 21,
            },
            end: Position {
                line: 1,
                column: 26,
            },
        }
    } else {
        Location {
            begin: Position { line: 1, column: 8 },
            end: Position {
                line: 1,
                column: 26,
            },
        }
    };
    assert_eq!(expected_location, result.errors[0].location);

    let tm = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    assert_eq!("number", to_string_type_id(tm.wanted_type));
    assert_eq!("string", to_string_type_id(tm.given_type));
}
