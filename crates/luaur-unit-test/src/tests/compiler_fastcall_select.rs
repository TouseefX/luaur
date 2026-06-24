#[cfg(test)]
#[test]
fn compiler_fastcall_select() {
    use crate::functions::compile_function_0::compile_function_0;

    let result1 = compile_function_0("return (select('#', ...))");
    let expected1 = "\nLOADK R1 K0 ['#']\nFASTCALL1 57 R1 L0\nGETIMPORT R0 2 [select]\nGETVARARGS R2 -1\nCALL R0 -1 1\nL0: RETURN R0 1\n";
    assert_eq!("\n".to_string() + &result1, expected1);

    let result2 = compile_function_0(
        r#"local sum = 0
for i=1, select('#', ...) do
    sum += select(i, ...)
end
return sum
"#,
    );
    let expected2 = "\nLOADN R0 0\nLOADN R3 1\nLOADK R5 K0 ['#']\nFASTCALL1 57 R5 L0\nGETIMPORT R4 2 [select]\nGETVARARGS R6 -1\nCALL R4 -1 1\nL0: MOVE R1 R4\nLOADN R2 1\nFORNPREP R1 L3\nL1: FASTCALL1 57 R3 L2\nGETIMPORT R4 2 [select]\nMOVE R5 R3\nGETVARARGS R6 -1\nCALL R4 -1 1\nL2: ADD R0 R0 R4\nFORNLOOP R1 L1\nL3: RETURN R0 1\n";
    assert_eq!("\n".to_string() + &result2, expected2);

    let result3 = compile_function_0("return select('#', ...)");
    let expected3 = "\nGETIMPORT R0 1 [select]\nLOADK R1 K2 ['#']\nGETVARARGS R2 -1\nCALL R0 -1 -1\nRETURN R0 -1\n";
    assert_eq!("\n".to_string() + &result3, expected3);

    let result4 = compile_function_0("return select('#')");
    let expected4 = "\nGETIMPORT R0 1 [select]\nLOADK R1 K2 ['#']\nCALL R0 1 -1\nRETURN R0 -1\n";
    assert_eq!("\n".to_string() + &result4, expected4);

    let result5 = compile_function_0("return select('#', foo())");
    let expected5 = "\nGETIMPORT R0 1 [select]\nLOADK R1 K2 ['#']\nGETIMPORT R2 4 [foo]\nCALL R2 0 -1\nCALL R0 -1 -1\nRETURN R0 -1\n";
    assert_eq!("\n".to_string() + &result5, expected5);
}
