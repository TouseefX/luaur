#[cfg(test)]
#[test]
fn compiler_preserve_neg_zero() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0("return 0");
    let expected = "\nLOADN R0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("return -0");
    let expected = "\nLOADK R0 K0 [-0]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result), expected);
}
