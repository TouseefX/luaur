//! Ported from `tests/TypeInfer.aliases.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_aliases_exported_type_function_location_is_accessible_on_module() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        export type function Apply()
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let module = unsafe { &*fixture.get_main_module(false) };
    let tfun = module
        .exported_type_bindings
        .get("Apply")
        .expect("expected exported type function Apply");
    assert_eq!(
        Some(Location {
            begin: Position { line: 1, column: 8 },
            end: Position {
                line: 2,
                column: 11,
            },
        }),
        tfun.definition_location()
    );
}
