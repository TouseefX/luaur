#[cfg(test)]
#[test]
fn compiler_conditional_basic() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual1 = compile_function_0("local a = ... if a then return 5 end");
    let expected1 =
        "\nGETVARARGS R0 1\nJUMPIFNOT R0 L0\nLOADN R1 5\nRETURN R1 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function_0("local a = ... if not a then return 5 end");
    let expected2 = "\nGETVARARGS R0 1\nJUMPIF R0 L0\nLOADN R1 5\nRETURN R1 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
