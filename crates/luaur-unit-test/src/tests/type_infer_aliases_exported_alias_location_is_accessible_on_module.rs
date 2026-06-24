//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_exported_alias_location_is_accessible_on_module() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type Value = string
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let module = unsafe { &*fixture.get_main_module(false) };
    let tfun = module
        .exported_type_bindings
        .get("Value")
        .expect("expected exported type Value");
    assert_eq!(
        Some(Location {
            begin: Position { line: 1, column: 8 },
            end: Position {
                line: 1,
                column: 34,
            },
        }),
        tfun.definition_location()
    );
}
