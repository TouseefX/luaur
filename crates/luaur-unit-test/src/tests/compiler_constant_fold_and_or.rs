#[cfg(test)]
#[test]
fn compiler_constant_fold_and_or() {
    use crate::functions::compile_function_0::compile_function_0;

    let result1 = compile_function_0("return true and 2");
    let expected1 = "\nLOADN R0 2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_function_0("return false and 2");
    let expected2 = "\nLOADB R0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result2), expected2);

    let result3 = compile_function_0("return nil and 2");
    let expected3 = "\nLOADNIL R0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result3), expected3);

    let result4 = compile_function_0("return true or 2");
    let expected4 = "\nLOADB R0 1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result4), expected4);

    let result5 = compile_function_0("return false or 2");
    let expected5 = "\nLOADN R0 2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result5), expected5);

    let result6 = compile_function_0("return nil or 2");
    let expected6 = "\nLOADN R0 2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result6), expected6);

    let result7 = compile_function_0("return true and a");
    let expected7 = "\nGETIMPORT R0 1 [a]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result7), expected7);

    let result8 = compile_function_0("return false and a");
    let expected8 = "\nLOADB R0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result8), expected8);

    let result9 = compile_function_0("return true or a");
    let expected9 = "\nLOADB R0 1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result9), expected9);

    let result10 = compile_function_0("return false or a");
    let expected10 = "\nGETIMPORT R0 1 [a]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result10), expected10);

    let result11 = compile_function_0("return a and true and b");
    let expected11 = "\nGETIMPORT R0 1 [a]\nJUMPIFNOT R0 L0\nGETIMPORT R0 3 [b]\nL0: RETURN R0 1\n";
    assert_eq!(format!("\n{}", result11), expected11);

    let result12 = compile_function_0("return a or false or b");
    let expected12 = "\nGETIMPORT R0 1 [a]\nJUMPIF R0 L0\nGETIMPORT R0 3 [b]\nL0: RETURN R0 1\n";
    assert_eq!(format!("\n{}", result12), expected12);
}
