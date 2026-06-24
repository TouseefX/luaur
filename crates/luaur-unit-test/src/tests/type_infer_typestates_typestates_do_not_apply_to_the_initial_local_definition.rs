//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_typestates_do_not_apply_to_the_initial_local_definition() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_ast::records::position::Position;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    if FFlag::DebugLuauForceOldSolver.get() {
        return;
    }

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        type MyType = number | string
        local foo: MyType = 5
        print(foo)
        foo = 7
        print(foo)
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "number | string",
        to_string_type_id_to_string_options(
            fixture.base.require_type_at_position_position(Position {
                line: 3,
                column: 14
            }),
            &mut opts,
        )
    );
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    assert_eq!(
        "number",
        to_string_type_id_to_string_options(
            fixture.base.require_type_at_position_position(Position {
                line: 5,
                column: 14
            }),
            &mut opts,
        )
    );
}
