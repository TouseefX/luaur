#[cfg(test)]
#[test]
fn compiler_constant_fold_arith() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0("return 10 + 2");
    let expected = "\nLOADN R0 12\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("return 10 - 2");
    let expected = "\nLOADN R0 8\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("return 10 * 2");
    let expected = "\nLOADN R0 20\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("return 10 / 2");
    let expected = "\nLOADN R0 5\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("return 10 % 2");
    let expected = "\nLOADN R0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("return 10 ^ 2");
    let expected = "\nLOADN R0 100\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("return -(2 - 5)");
    let expected = "\nLOADN R0 3\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("return (2 + 2) * 2");
    let expected = "\nLOADN R0 8\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);
}
