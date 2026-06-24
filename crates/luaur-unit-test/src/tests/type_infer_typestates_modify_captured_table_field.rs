//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_modify_captured_table_field() {
    use crate::records::fixture::Fixture;
    use alloc::string::String;
    use luaur_analysis::functions::to_string_to_string_alt_m::to_string_type_id_to_string_options;
    use luaur_analysis::records::to_string_options::ToStringOptions;
    use luaur_common::FFlag;

    let mut fixture = Fixture::fixture_bool(false);

    let result = fixture.check_string_optional_frontend_options(
        &String::from(
            r#"
        local state = { x = 0 }
        function incr()
            state.x = state.x + 1
        end
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
    let rand_ty = fixture
        .get_type(&String::from("state"), false)
        .expect("expected state type");
    let mut opts = ToStringOptions::default();
    opts.exhaustive = true;
    if !FFlag::DebugLuauForceOldSolver.get() {
        assert_eq!(
            "{ x: number }",
            to_string_type_id_to_string_options(rand_ty, &mut opts)
        );
    } else {
        assert_eq!(
            "{| x: number |}",
            to_string_type_id_to_string_options(rand_ty, &mut opts)
        );
    }
}
