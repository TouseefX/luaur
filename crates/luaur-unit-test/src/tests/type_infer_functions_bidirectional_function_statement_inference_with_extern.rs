#[cfg(test)]
#[test]
fn type_infer_functions_bidirectional_function_statement_inference_with_extern() {
    use crate::records::extern_type_fixture::ExternTypeFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_ast::records::position::Position;

    let mut fixture = ExternTypeFixture::default();
    fixture.get_frontend();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type HasClass = { f: (ClassWithGenericMethod) -> () }
        local t = {} :: HasClass
        function t.f(cls)
            local _ = cls
            local foobar = cls.identity(42)
            local _ = foobar
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    assert_eq!(
        "ClassWithGenericMethod",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position {
                    line: 4,
                    column: 23
                })
        )
    );
    assert_eq!(
        "number",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position {
                    line: 6,
                    column: 23
                })
        )
    );
}
