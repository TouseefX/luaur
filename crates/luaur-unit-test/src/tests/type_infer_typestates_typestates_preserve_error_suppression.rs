//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_typestates_preserve_error_suppression() {
    use crate::records::type_state_fixture::TypeStateFixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_ast::records::position::Position;

    let mut fixture = TypeStateFixture::default();

    let result = fixture.base.base.check_string_optional_frontend_options(
        &String::from(r#"
        local a: any = 51
        a = "pickles" -- We'll have a new DefId for this iteration of `a`.  Its type must also be error-suppressing
        print(a)
    "#),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "*error-type* | string",
        to_string_type_id_to_string_options(
            fixture
                .base
                .base
                .require_type_at_position_position(Position {
                    line: 3,
                    column: 14
                }),
            &mut opts,
        )
    );
}
