#[cfg(test)]
#[test]
fn compiler_interp_string_register_limit() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::functions::rep::rep;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauCompileStringInterpTargetTop;

    let _scoped_flag = ScopedFastFlag::new(&LuauCompileStringInterpTargetTop, true);

    // C++ CHECK_THROWS_AS: 254 interpolations exceed the register limit -> CompileError.
    let source = format!("local a = `{}`", rep("{1}", 254));
    let threw = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = compile_function_0(&source);
    }))
    .is_err();
    assert!(
        threw,
        "Expected 254 interpolations to exceed the register limit"
    );

    // C++ CHECK_NOTHROW: 253 interpolations still fit.
    let source_253 = format!("local a = `{}`", rep("{1}", 253));
    let ok = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let _ = compile_function_0(&source_253);
    }))
    .is_ok();
    assert!(ok, "Expected 253 interpolations to compile without error");
}
