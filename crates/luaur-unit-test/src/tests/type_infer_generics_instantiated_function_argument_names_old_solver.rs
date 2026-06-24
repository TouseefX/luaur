#[cfg(test)]
#[test]
fn type_infer_generics_instantiated_function_argument_names_old_solver() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_ast::records::position::Position;

    crate::DOES_NOT_PASS_NEW_SOLVER_GUARD!();

    let mut fixture = Fixture::fixture_bool(false);
    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local function f<T, U...>(a: T, ...: U...) end

        f(1, 2, 3)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);

    let ty = fixture
        .find_type_at_position_position(Position { line: 3, column: 8 })
        .expect("expected type at position");
    let mut opts = ToStringOptions::default();
    opts.function_type_arguments = true;
    assert_eq!(
        "(a: number, number, number) -> ()",
        to_string_type_id_to_string_options(ty, &mut opts)
    );
}
