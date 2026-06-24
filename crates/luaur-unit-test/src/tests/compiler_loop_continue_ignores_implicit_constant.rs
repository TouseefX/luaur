#[cfg(test)]
#[test]
fn compiler_loop_continue_ignores_implicit_constant() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0("local _\nrepeat\ncontinue\nuntil not _\n");
    let expected = "\nRETURN R0 0\nRETURN R0 0\n";

    assert_eq!(format!("\n{}", result), expected);
}
