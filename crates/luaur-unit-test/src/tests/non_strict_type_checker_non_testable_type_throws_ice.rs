//! Ported from `tests/NonStrictTypeChecker.test.cpp`.

#[cfg(test)]
#[test]
fn non_strict_type_checker_non_testable_type_throws_ice() {
    use crate::records::non_strict_type_checker_fixture::NonStrictTypeCheckerFixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::internal_compiler_error::InternalCompilerError;
    use luaur_common::FFlag;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let _force_old_solver = ScopedFastFlag::new(&FFlag::DebugLuauForceOldSolver, false);
    let mut fixture = NonStrictTypeCheckerFixture::default();

    let result = catch_unwind(AssertUnwindSafe(|| {
        fixture.check_non_strict(&String::from(
            r#"os.time({year = 0, month = 0, day = 0, min = 0, isdst = nil})
"#,
        ));
    }));

    let payload = result.expect_err("expected InternalCompilerError");
    assert!(
        payload.is::<InternalCompilerError>(),
        "expected InternalCompilerError panic payload"
    );
}
