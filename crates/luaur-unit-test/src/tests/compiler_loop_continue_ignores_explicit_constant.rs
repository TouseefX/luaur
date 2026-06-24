#[cfg(test)]
#[test]
fn compiler_loop_continue_ignores_explicit_constant() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0("local c = true\nrepeat\n    continue\nuntil c");
    let expected = "\nRETURN R0 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
