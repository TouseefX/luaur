#[cfg(test)]
#[test]
fn type_infer_functions_table_annotated_explicit_self() {
    use crate::functions::type_error_data_ref::type_error_data_ref;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::records::function_exits_without_returning::FunctionExitsWithoutReturning;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        type MyObject = {
            fn: (self: MyObject) -> number,
            field: number
        }

        local Foo = {} :: MyObject

        function Foo:fn()
            local _ = self
        end
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    type_error_data_ref::<FunctionExitsWithoutReturning>(&result.errors[0])
        .expect("expected FunctionExitsWithoutReturning");
    assert_eq!(
        "MyObject",
        to_string_type_id(fixture.require_type_at_position_position(Position {
            line: 9,
            column: 24
        }))
    );
}
