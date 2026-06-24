#[cfg(test)]
#[test]
fn compiler_fastcall_bytecode() {
    use crate::functions::compile_function_0::compile_function_0;

    // direct global call
    let result = compile_function_0("return math.abs(-5)");
    let expected = "\nLOADN R1 -5\nFASTCALL1 2 R1 L0\nGETIMPORT R0 2 [math.abs]\nCALL R0 1 -1\nL0: RETURN R0 -1\n";
    assert_eq!(format!("\n{}", result), expected);

    // call through a local variable
    let result = compile_function_0("local abs = math.abs return abs(-5)");
    let expected = "\nGETIMPORT R0 2 [math.abs]\nLOADN R2 -5\nFASTCALL1 2 R2 L0\nMOVE R1 R0\nCALL R1 1 -1\nL0: RETURN R1 -1\n";
    assert_eq!(format!("\n{}", result), expected);

    // call through an upvalue
    let result =
        compile_function_0("local abs = math.abs function foo() return abs(-5) end return foo()");
    let expected =
        "\nLOADN R1 -5\nFASTCALL1 2 R1 L0\nGETUPVAL R0 0\nCALL R0 1 -1\nL0: RETURN R0 -1\n";
    assert_eq!(format!("\n{}", result), expected);

    // mutating the global in the script breaks the optimization
    let result = compile_function_0("math = {} return math.abs(-5)");
    let expected = "\nNEWTABLE R0 0 0\nSETGLOBAL R0 K0 ['math']\nGETGLOBAL R0 K0 ['math']\nGETTABLEKS R0 R0 K1 ['abs']\nLOADN R1 -5\nCALL R0 1 -1\nRETURN R0 -1\n";
    assert_eq!(format!("\n{}", result), expected);

    // mutating the local in the script breaks the optimization
    let result = compile_function_0("local abs = math.abs abs = nil return abs(-5)");
    let expected = "\nGETIMPORT R0 2 [math.abs]\nLOADNIL R0\nMOVE R1 R0\nLOADN R2 -5\nCALL R1 1 -1\nRETURN R1 -1\n";
    assert_eq!(format!("\n{}", result), expected);

    // mutating the global in the script breaks the optimization, even if you do this after computing the local (for simplicity)
    let result = compile_function_0("local abs = math.abs math = {} return abs(-5)");
    let expected = "\nGETGLOBAL R0 K0 ['math']\nGETTABLEKS R0 R0 K1 ['abs']\nNEWTABLE R1 0 0\nSETGLOBAL R1 K0 ['math']\nMOVE R1 R0\nLOADN R2 -5\nCALL R1 1 -1\nRETURN R1 -1\n";
    assert_eq!(format!("\n{}", result), expected);
}
