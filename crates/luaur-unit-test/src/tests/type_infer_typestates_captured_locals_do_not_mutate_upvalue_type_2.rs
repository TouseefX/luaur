//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_captured_locals_do_not_mutate_upvalue_type_2() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;
    use luaur_analysis::functions::get_error::get_type_error;
    use luaur_analysis::functions::to_string_to_string_alt_c::to_string_type_id;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_analysis::records::type_mismatch::TypeMismatch;
    use luaur_ast::records::position::Position;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = {x = nil}

        function f()
            print(t.x)
            t = {x = "five"}
        end

        t = {x = 5}
        f()
    "#,
        ),
        None,
    );

    assert_eq!(1, result.errors.len(), "{:?}", result.errors);
    let err = unsafe { get_type_error::<TypeMismatch>(&result.errors[0]).as_ref() }
        .expect("expected TypeMismatch");
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "{ x: nil } | { x: number }",
        to_string_type_id_to_string_options(err.wanted_type, &mut opts)
    );
    assert_eq!("{ x: string }", to_string_type_id(err.given_type));
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "{ x: nil } | { x: number }",
        to_string_type_id_to_string_options(
            fixture
                .base
                .base
                .require_type_at_position_position(Position {
                    line: 4,
                    column: 18
                }),
            &mut opts,
        )
    );
    assert_eq!(
        "number?",
        to_string_type_id(
            fixture
                .base
                .base
                .require_type_at_position_position(Position {
                    line: 4,
                    column: 20,
                })
        )
    );
}
