//! Ported from `tests/TypeInfer.typestates.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_typestates_fuzzer_table_freeze_in_conditional_expr() {
    use crate::records::builtins_fixture::BuiltinsFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::internal_compiler_error::InternalCompilerError;
    use luaur_common::FFlag;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let _new_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = BuiltinsFixture::default();
    fixture.get_frontend();

    let result = catch_unwind(AssertUnwindSafe(|| {
        fixture.base.check_string_optional_frontend_options(
            &String::from(
                r#"
            local _
            if
                if table.freeze(_,_) then _ else _
            then
            end
        "#,
            ),
            None,
        )
    }));

    let payload = result.expect_err("expected InternalCompilerError");
    assert!(
        payload.is::<InternalCompilerError>(),
        "expected InternalCompilerError panic payload"
    );
}
