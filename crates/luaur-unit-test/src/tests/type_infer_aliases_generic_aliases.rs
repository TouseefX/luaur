//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_generic_aliases() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_error::to_string_type_error;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _sff = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T<a> = { v: a }
        local x: T<number> = { v = 123 }
        local y: T<string> = { v = "foo" }
        local bad: T<number> = { v = "foo" }
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 4,
                column: 37,
            },
            end: Position {
                line: 4,
                column: 42,
            },
        },
        result.errors[0].location
    );
    assert_eq!(
        "Expected this to be 'number', but got 'string'",
        to_string_type_error(&result.errors[0])
    );
}
