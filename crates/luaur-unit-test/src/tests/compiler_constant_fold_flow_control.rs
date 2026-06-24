#[cfg(test)]
#[test]
fn compiler_constant_fold_flow_control() {
    use crate::functions::compile_function_0::compile_function_0;

    let result1 = compile_function_0("if true then print(1) end");
    let expected1 = "\nGETIMPORT R0 1 [print]\nLOADN R1 1\nCALL R0 1 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_function_0("if false then print(1) end");
    let expected2 = "\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result2), expected2);

    let result3 = compile_function_0("if true then print(1) else print(2) end");
    let expected3 = "\nGETIMPORT R0 1 [print]\nLOADN R1 1\nCALL R0 1 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result3), expected3);

    let result4 = compile_function_0("if false then print(1) else print(2) end");
    let expected4 = "\nGETIMPORT R0 1 [print]\nLOADN R1 2\nCALL R0 1 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result4), expected4);

    let result5 = compile_function_0("while true do print(1) end");
    let expected5 =
        "\nL0: GETIMPORT R0 1 [print]\nLOADN R1 1\nCALL R0 1 0\nJUMPBACK L0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result5), expected5);

    let result6 = compile_function_0("while false do print(1) end");
    let expected6 = "\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result6), expected6);

    let result7 = compile_function_0("repeat print(1) until true");
    let expected7 = "\nGETIMPORT R0 1 [print]\nLOADN R1 1\nCALL R0 1 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result7), expected7);

    let result8 = compile_function_0("repeat print(1) until false");
    let expected8 =
        "\nL0: GETIMPORT R0 1 [print]\nLOADN R1 1\nCALL R0 1 0\nJUMPBACK L0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result8), expected8);

    let result9 = compile_function_0("repeat print(1) until five and false");
    let expected9 = "\nL0: GETIMPORT R0 1 [print]\nLOADN R1 1\nCALL R0 1 0\nGETIMPORT R0 3 [five]\nJUMPIFNOT R0 L1\nL1: JUMPBACK L0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result9), expected9);
}
