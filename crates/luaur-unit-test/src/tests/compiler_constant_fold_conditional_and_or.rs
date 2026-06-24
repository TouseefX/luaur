#[cfg(test)]
#[test]
fn compiler_constant_fold_conditional_and_or() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual1 = compile_function_0("local a = ... if false or a then print(1) end");
    let expected1 = "\nGETVARARGS R0 1\nJUMPIFNOT R0 L0\nGETIMPORT R1 1 [print]\nLOADN R2 1\nCALL R1 1 0\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function_0("local a = ... if not (false or a) then print(1) end");
    let expected2 = "\nGETVARARGS R0 1\nJUMPIF R0 L0\nGETIMPORT R1 1 [print]\nLOADN R2 1\nCALL R1 1 0\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    let actual3 = compile_function_0("local a = ... if true and a then print(1) end");
    let expected3 = "\nGETVARARGS R0 1\nJUMPIFNOT R0 L0\nGETIMPORT R1 1 [print]\nLOADN R2 1\nCALL R1 1 0\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual3), expected3);

    let actual4 = compile_function_0("local a = ... if not (true and a) then print(1) end");
    let expected4 = "\nGETVARARGS R0 1\nJUMPIF R0 L0\nGETIMPORT R1 1 [print]\nLOADN R2 1\nCALL R1 1 0\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual4), expected4);
}
