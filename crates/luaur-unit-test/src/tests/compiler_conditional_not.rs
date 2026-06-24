#[cfg(test)]
#[test]
fn compiler_conditional_not() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0("local a, b = ... if not (not (a < b)) then return 5 end");
    let expected =
        "\nGETVARARGS R0 2\nJUMPIFNOTLT R0 R1 L0\nLOADN R2 5\nRETURN R2 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual =
        compile_function_0("local a, b = ... if not (not (not (a < b))) then return 5 end");
    let expected =
        "\nGETVARARGS R0 2\nJUMPIFLT R0 R1 L0\nLOADN R2 5\nRETURN R2 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
