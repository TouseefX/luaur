#[cfg(test)]
#[test]
fn compiler_interp_string_with_no_expressions() {
    use crate::functions::compile_function_0::compile_function_0;

    assert_eq!(
        compile_function_0(r#"return "hello""#),
        compile_function_0(r#"return `hello`"#)
    );
}
