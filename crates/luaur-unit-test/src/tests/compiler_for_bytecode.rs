#[cfg(test)]
#[test]
fn compiler_for_bytecode() {
    use crate::functions::compile_function_0::compile_function_0;

    // basic for loop: variable directly refers to internal iteration index (R2)
    let actual = compile_function_0("for i=1,5 do print(i) end");
    let expected = "\nLOADN R2 1\nLOADN R0 5\nLOADN R1 1\nFORNPREP R0 L1\nL0: GETIMPORT R3 1 [print]\nMOVE R4 R2\nCALL R3 1 0\nFORNLOOP R0 L0\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // when you assign the variable internally, we freak out and copy the variable so that you aren't changing the loop behavior
    let actual = compile_function_0("for i=1,5 do i = 7 print(i) end");
    let expected = "\nLOADN R2 1\nLOADN R0 5\nLOADN R1 1\nFORNPREP R0 L1\nL0: MOVE R3 R2\nLOADN R3 7\nGETIMPORT R4 1 [print]\nMOVE R5 R3\nCALL R4 1 0\nFORNLOOP R0 L0\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // basic for-in loop, generic version
    let actual = compile_function_0(
        "for word in string.gmatch(\"Hello Lua user\", \"%a+\") do print(word) end",
    );
    let expected = "\nGETIMPORT R0 2 [string.gmatch]\nLOADK R1 K3 ['Hello Lua user']\nLOADK R2 K4 ['%a+']\nCALL R0 2 3\nFORGPREP R0 L1\nL0: GETIMPORT R5 6 [print]\nMOVE R6 R3\nCALL R5 1 0\nL1: FORGLOOP R0 L0 1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // basic for-in loop, using inext specialization
    let actual = compile_function_0("for k,v in ipairs({}) do print(k,v) end");
    let expected = "\nGETIMPORT R0 1 [ipairs]\nNEWTABLE R1 0 0\nCALL R0 1 3\nFORGPREP_INEXT R0 L1\nL0: GETIMPORT R5 3 [print]\nMOVE R6 R3\nMOVE R7 R4\nCALL R5 2 0\nL1: FORGLOOP R0 L0 2 [inext]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // basic for-in loop, using next specialization
    let actual = compile_function_0("for k,v in pairs({}) do print(k,v) end");
    let expected = "\nGETIMPORT R0 1 [pairs]\nNEWTABLE R1 0 0\nCALL R0 1 3\nFORGPREP_NEXT R0 L1\nL0: GETIMPORT R5 3 [print]\nMOVE R6 R3\nMOVE R7 R4\nCALL R5 2 0\nL1: FORGLOOP R0 L0 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("for k,v in next,{} do print(k,v) end");
    let expected = "\nGETIMPORT R0 1 [next]\nNEWTABLE R1 0 0\nLOADNIL R2\nFORGPREP_NEXT R0 L1\nL0: GETIMPORT R5 3 [print]\nMOVE R6 R3\nMOVE R7 R4\nCALL R5 2 0\nL1: FORGLOOP R0 L0 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
