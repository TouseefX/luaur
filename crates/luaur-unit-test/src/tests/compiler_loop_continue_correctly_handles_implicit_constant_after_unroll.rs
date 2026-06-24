#[cfg(test)]
#[test]
fn compiler_loop_continue_correctly_handles_implicit_constant_after_unroll() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use alloc::string::ToString;
    use luaur_common::FInt;
    use luaur_compiler::records::compile_error::CompileError;

    let _scoped_int = ScopedFastInt::new(&FInt::LuauCompileLoopUnrollThreshold, 200);

    // C++ source is an R"(...)" literal with a leading newline, so the reported line
    // numbers (continue on line 6, condition on line 9) are 1 greater than the body lines.
    let result = std::panic::catch_unwind(|| {
        compile_function(
            r#"
for i = 1, 2 do
    s()
    repeat
        if i == 2 then
            continue
        end
        local x = i == 1 or a
    until f(x)
end
"#,
            0,
            2,
            0,
        )
    });

    assert!(result.is_err(), "Expected CompileError");

    let err = result.unwrap_err();
    let err_obj = err
        .downcast_ref::<CompileError>()
        .expect("panic payload is not a CompileError");

    assert_eq!(err_obj.get_location().begin.line + 1, 9);

    let msg = unsafe {
        core::ffi::CStr::from_ptr(err_obj.what())
            .to_string_lossy()
            .to_string()
    };
    assert_eq!(
        msg,
        "Local x used in the repeat..until condition is undefined because continue statement on line 6 jumps over it"
    );
}
