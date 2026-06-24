//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag() {
    use crate::records::assertion_catcher::AssertionCatcher;
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::string::String;
    use luaur_analysis::records::internal_compiler_error::InternalCompilerError;
    use luaur_common::assert_call_handler;
    use luaur_common::FFlag;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let _ac = AssertionCatcher::new();
    let mut fixture = Fixture::fixture_bool(false);

    let result = catch_unwind(AssertUnwindSafe(|| {
        fixture.check_string_optional_frontend_options(
            &String::from(
                r#"
        local a: _luau_ice = 55
    "#,
            ),
            None,
        );
    }));

    let panic = result.expect_err("expected InternalCompilerError");
    assert!(
        panic.downcast_ref::<InternalCompilerError>().is_some(),
        "expected InternalCompilerError panic payload"
    );

    if AssertionCatcher::tripped() != 1 {
        assert_call_handler(
            c"1 == AssertionCatcher::tripped".as_ptr(),
            c"TypeInfer.annotations.test.cpp".as_ptr(),
            line!() as i32,
            c"type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag".as_ptr(),
        );
    }
}
