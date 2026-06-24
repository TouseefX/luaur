//! Ported from `tests/TypeInfer.annotations.test.cpp`.

#[cfg(test)]
#[test]
fn type_infer_annotations_luau_ice_triggers_an_ice_exception_with_flag_handler() {
    use crate::records::fixture::Fixture;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use alloc::rc::Rc;
    use alloc::string::String;
    use core::cell::Cell;
    use luaur_analysis::records::internal_compiler_error::InternalCompilerError;
    use luaur_common::FFlag;
    use std::panic::{catch_unwind, AssertUnwindSafe};

    let _sffs = ScopedFastFlag::new(&FFlag::DebugLuauMagicTypes, true);
    let caught = Rc::new(Cell::new(false));
    let caught_for_handler = caught.clone();
    let mut fixture = Fixture::fixture_bool(false);

    fixture.get_frontend().ice_handler.on_internal_error = Some(Rc::new(move |_| {
        caught_for_handler.set(true);
    }));

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
    assert!(caught.get());
}
