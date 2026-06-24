//! Ported from `tests/TypeInfer.modules.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_modules_type_error_of_unknown_qualified_type() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::records::unknown_symbol::{Context, UnknownSymbol};
    use luaur_ast::records::location::Location;
    use luaur_ast::records::position::Position;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local p: SomeModule.DoesNotExist
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        Location {
            begin: Position {
                line: 1,
                column: 17
            },
            end: Position {
                line: 1,
                column: 40
            }
        },
        result.errors[0].location
    );
    let error =
        type_error_data_ref::<UnknownSymbol>(&result.errors[0]).expect("expected UnknownSymbol");
    assert_eq!("SomeModule.DoesNotExist", error.name());
    assert_eq!(Context::Type, error.context());
}
