#[cfg(test)]
#[test]
fn compiler_if_else_expression() {
    use crate::functions::compile_function_0::compile_function_0;

    // codegen for a true constant condition
    let actual = compile_function_0("return if true then 10 else 20");
    let expected = "\nLOADN R0 10\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen for a false constant condition
    let actual = compile_function_0("return if false then 10 else 20");
    let expected = "\nLOADN R0 20\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen for a true constant condition with non-constant expressions
    let actual = compile_function_0("return if true then {} else error()");
    let expected = "\nNEWTABLE R0 0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen for a false constant condition with non-constant expressions
    let actual = compile_function_0("return if false then error() else {}");
    let expected = "\nNEWTABLE R0 0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen for a false (in this case 'nil') constant condition
    let actual = compile_function_0("return if nil then 10 else 20");
    let expected = "\nLOADN R0 20\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen constant if-else expression used with a binary operation involving another constant
    // The test verifies that everything constant folds down to a single constant
    let actual = compile_function_0("return 7 + if true then 10 else 20");
    let expected = "\nLOADN R0 17\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen for a non-constant condition
    let actual = compile_function_0("return if condition then 10 else 20");
    let expected = "\nGETIMPORT R1 1 [condition]\nJUMPIFNOT R1 L0\nLOADN R0 10\nRETURN R0 1\nL0: LOADN R0 20\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen for a non-constant condition using an assignment
    let actual = compile_function_0("result = if condition then 10 else 20");
    let expected = "\nGETIMPORT R1 1 [condition]\nJUMPIFNOT R1 L0\nLOADN R0 10\nJUMP L1\nL0: LOADN R0 20\nL1: SETGLOBAL R0 K2 ['result']\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen for a non-constant condition using an assignment to a local variable
    let actual = compile_function_0("local result = if condition then 10 else 20");
    let expected = "\nGETIMPORT R1 1 [condition]\nJUMPIFNOT R1 L0\nLOADN R0 10\nRETURN R0 0\nL0: LOADN R0 20\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // codegen for an if-else expression with multiple elseif's
    let actual = compile_function_0("result = if condition1 then 10 elseif condition2 then 20 elseif condition3 then 30 else 40");
    let expected = "\nGETIMPORT R1 1 [condition1]\nJUMPIFNOT R1 L0\nLOADN R0 10\nJUMP L3\nL0: GETIMPORT R1 3 [condition2]\nJUMPIFNOT R1 L1\nLOADN R0 20\nJUMP L3\nL1: GETIMPORT R1 5 [condition3]\nJUMPIFNOT R1 L2\nLOADN R0 30\nJUMP L3\nL2: LOADN R0 40\nL3: SETGLOBAL R0 K6 ['result']\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
