//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_type_alias_locations() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::find_scope_at_position::find_scope_at_position;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type T = number

        do
            type T = string
            type X = boolean
        end
    "#,
        ),
        None,
    );
    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let module = unsafe { &*fixture.get_main_module(false) };
    assert!(!module.scopes.is_empty());

    let module_scope = module.scopes[0].1.clone();
    assert_eq!(
        Some(&Location {
            begin: Position {
                line: 1,
                column: 13,
            },
            end: Position {
                line: 1,
                column: 14,
            },
        }),
        module_scope.type_alias_name_locations.get("T")
    );

    let do_scope =
        find_scope_at_position(module, Position { line: 4, column: 0 }).expect("expected do scope");
    assert_eq!(
        Some(&Location {
            begin: Position {
                line: 4,
                column: 17,
            },
            end: Position {
                line: 4,
                column: 18,
            },
        }),
        do_scope.type_alias_name_locations.get("T")
    );
    assert_eq!(
        Some(&Location {
            begin: Position {
                line: 5,
                column: 17,
            },
            end: Position {
                line: 5,
                column: 18,
            },
        }),
        do_scope.type_alias_name_locations.get("X")
    );
}
