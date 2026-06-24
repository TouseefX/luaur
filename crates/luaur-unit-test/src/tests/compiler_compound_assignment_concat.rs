#[cfg(test)]
#[test]
fn compiler_compound_assignment_concat() {
    use crate::functions::compile_function_0::compile_function_0;

    // basic concat
    let result = compile_function_0("local a = '' a ..= 'a'");
    let expected =
        "\nLOADK R0 K0 ['']\nMOVE R1 R0\nLOADK R2 K1 ['a']\nCONCAT R0 R1 R2\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &result, expected);

    // concat chains
    let result = compile_function_0("local a = '' a ..= 'a' .. 'b'");
    let expected = "\nLOADK R0 K0 ['']\nMOVE R1 R0\nLOADK R2 K1 ['a']\nLOADK R3 K2 ['b']\nCONCAT R0 R1 R3\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &result, expected);

    let result = compile_function_0("local a = '' a ..= 'a' .. 'b' .. 'c'");
    let expected = "\nLOADK R0 K0 ['']\nMOVE R1 R0\nLOADK R2 K1 ['a']\nLOADK R3 K2 ['b']\nLOADK R4 K3 ['c']\nCONCAT R0 R1 R4\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &result, expected);

    // concat on non-local
    let result = compile_function_0("_VERSION ..= 'a' .. 'b'");
    let expected = "\nGETGLOBAL R1 K0 ['_VERSION']\nLOADK R2 K1 ['a']\nLOADK R3 K2 ['b']\nCONCAT R0 R1 R3\nSETGLOBAL R0 K0 ['_VERSION']\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &result, expected);
}
