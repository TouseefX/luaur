#[cfg(test)]
#[test]
fn compiler_constant_jump_compare() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual1 = compile_function_0("local obj = ...\nlocal b = obj == 1");
    let expected1 = "\nGETVARARGS R0 1\nJUMPXEQKN R0 K0 L0 [1]\nLOADB R1 0 +1\nL0: LOADB R1 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function_0("local obj = ...\nlocal b = 1 == obj");
    let expected2 = "\nGETVARARGS R0 1\nJUMPXEQKN R0 K0 L0 [1]\nLOADB R1 0 +1\nL0: LOADB R1 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    let actual3 = compile_function_0("local obj = ...\nlocal b = \"Hello, Sailor!\" == obj");
    let expected3 = "\nGETVARARGS R0 1\nJUMPXEQKS R0 K0 L0 ['Hello, Sailor!']\nLOADB R1 0 +1\nL0: LOADB R1 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual3), expected3);

    let actual4 = compile_function_0("local obj = ...\nlocal b = nil == obj");
    let expected4 =
        "\nGETVARARGS R0 1\nJUMPXEQKNIL R0 L0\nLOADB R1 0 +1\nL0: LOADB R1 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual4), expected4);

    let actual5 = compile_function_0("local obj = ...\nlocal b = true == obj");
    let expected5 =
        "\nGETVARARGS R0 1\nJUMPXEQKB R0 1 L0\nLOADB R1 0 +1\nL0: LOADB R1 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual5), expected5);

    let actual6 = compile_function_0("local obj = ...\nlocal b = nil ~= obj");
    let expected6 = "\nGETVARARGS R0 1\nJUMPXEQKNIL R0 L0 NOT\nLOADB R1 0 +1\nL0: LOADB R1 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual6), expected6);

    let actual7 = compile_function_0("local obj = ...\nlocal b = obj == {}");
    let expected7 = "\nGETVARARGS R0 1\nNEWTABLE R2 0 0\nJUMPIFEQ R0 R2 L0\nLOADB R1 0 +1\nL0: LOADB R1 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual7), expected7);
}
