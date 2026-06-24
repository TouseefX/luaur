#[cfg(test)]
#[test]
fn compiler_if_elimination() {
    use crate::functions::compile_function_0::compile_function_0;

    let result1 = compile_function_0("local a = false if a and b then b() end");
    let expected1 = "\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_function_0("local a = true if a or b then b() end");
    let expected2 = "\nGETIMPORT R0 1 [b]\nCALL R0 0 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result2), expected2);

    let result3 = compile_function_0("local a = false if a and b then b() else return 42 end");
    let expected3 = "\nLOADN R0 42\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result3), expected3);

    let result4 = compile_function_0("local a = true if a or b then b() else return 42 end");
    let expected4 = "\nGETIMPORT R0 1 [b]\nCALL R0 0 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result4), expected4);

    let result5 = compile_function_0("local a = false if b and a then return 1 end");
    let expected5 = "\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result5), expected5);

    let result6 = compile_function_0("local a = false if b and a then return 1 else return 2 end");
    let expected6 = "\nLOADN R0 2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result6), expected6);

    let result7 = compile_function_0("local a = true if b and a then return 1 end");
    let expected7 =
        "\nGETIMPORT R0 1 [b]\nJUMPIFNOT R0 L0\nLOADN R0 1\nRETURN R0 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", result7), expected7);

    let result8 = compile_function_0("local a = true if b and a then return 1 else return 2 end");
    let expected8 = "\nGETIMPORT R0 1 [b]\nJUMPIFNOT R0 L0\nLOADN R0 1\nRETURN R0 1\nL0: LOADN R0 2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result8), expected8);

    let result9 = compile_function_0("local a = false if b.test and a then return 1 end");
    let expected9 = "\nGETIMPORT R0 2 [b.test]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result9), expected9);

    let result10 =
        compile_function_0("local a = false if b.test and a then return 1 else return 2 end");
    let expected10 = "\nGETIMPORT R0 2 [b.test]\nLOADN R0 2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result10), expected10);
}
