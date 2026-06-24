#[cfg(test)]
#[test]
fn compiler_conditional_and_or() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0("local a, b, c = ... if a < b and b < c then return 5 end");
    let expected = "\nGETVARARGS R0 3\nJUMPIFNOTLT R0 R1 L0\nJUMPIFNOTLT R1 R2 L0\nLOADN R3 5\nRETURN R3 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a, b, c = ... if a < b or b < c then return 5 end");
    let expected = "\nGETVARARGS R0 3\nJUMPIFLT R0 R1 L0\nJUMPIFNOTLT R1 R2 L1\nL0: LOADN R3 5\nRETURN R3 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual =
        compile_function_0("local a,b,c,d = ... if (a or b) and not (c and d) then return 5 end");
    let expected = "\nGETVARARGS R0 4\nJUMPIF R0 L0\nJUMPIFNOT R1 L2\nL0: JUMPIFNOT R2 L1\nJUMPIF R3 L2\nL1: LOADN R4 5\nRETURN R4 1\nL2: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a,b,c = ... if a or not b or c then return 5 end");
    let expected = "\nGETVARARGS R0 3\nJUMPIF R0 L0\nJUMPIFNOT R1 L0\nJUMPIFNOT R2 L1\nL0: LOADN R3 5\nRETURN R3 1\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a,b,c = ... if a and not b and c then return 5 end");
    let expected = "\nGETVARARGS R0 3\nJUMPIFNOT R0 L0\nJUMPIF R1 L0\nJUMPIFNOT R2 L0\nLOADN R3 5\nRETURN R3 1\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
