#[cfg(test)]
#[test]
fn compiler_conditional_compare() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual1 = compile_function_0("local a, b = ... if a < b then return 5 end");
    let expected1 =
        "\nGETVARARGS R0 2\nJUMPIFNOTLT R0 R1 L0\nLOADN R2 5\nRETURN R2 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function_0("local a, b = ... if a <= b then return 5 end");
    let expected2 =
        "\nGETVARARGS R0 2\nJUMPIFNOTLE R0 R1 L0\nLOADN R2 5\nRETURN R2 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    let actual3 = compile_function_0("local a, b = ... if a > b then return 5 end");
    let expected3 =
        "\nGETVARARGS R0 2\nJUMPIFNOTLT R1 R0 L0\nLOADN R2 5\nRETURN R2 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual3), expected3);

    let actual4 = compile_function_0("local a, b = ... if a >= b then return 5 end");
    let expected4 =
        "\nGETVARARGS R0 2\nJUMPIFNOTLE R1 R0 L0\nLOADN R2 5\nRETURN R2 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual4), expected4);

    let actual5 = compile_function_0("local a, b = ... if a == b then return 5 end");
    let expected5 =
        "\nGETVARARGS R0 2\nJUMPIFNOTEQ R0 R1 L0\nLOADN R2 5\nRETURN R2 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual5), expected5);

    let actual6 = compile_function_0("local a, b = ... if a ~= b then return 5 end");
    let expected6 =
        "\nGETVARARGS R0 2\nJUMPIFEQ R0 R1 L0\nLOADN R2 5\nRETURN R2 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual6), expected6);
}
