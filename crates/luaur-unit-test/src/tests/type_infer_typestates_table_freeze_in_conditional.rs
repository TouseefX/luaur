//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_table_freeze_in_conditional() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_common::FFlag;

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = fixture.base.check_string_optional_frontend_options(
        &String::from(
            r#"
        local t = { x = 42 }
        if math.random() > 0.5 and table.freeze(t) then
        end
        t.y = 13
    "#,
        ),
        None,
    );

    assert_eq!(0, result.errors.len(), "{:?}", result.errors);
}
