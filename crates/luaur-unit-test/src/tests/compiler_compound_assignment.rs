#[cfg(test)]
#[test]
fn compiler_compound_assignment() {
    use crate::functions::compile_function_0::compile_function_0;

    // globals vs constants
    let result = compile_function_0("a += 1");
    let expected =
        "\nGETGLOBAL R0 K0 ['a']\nADDK R0 R0 K1 [1]\nSETGLOBAL R0 K0 ['a']\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // globals vs expressions
    let result = compile_function_0("a -= a");
    let expected = "\nGETGLOBAL R0 K0 ['a']\nGETGLOBAL R1 K0 ['a']\nSUB R0 R0 R1\nSETGLOBAL R0 K0 ['a']\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // locals vs constants
    let result = compile_function_0("local a = 1 a *= 2");
    let expected = "\nLOADN R0 1\nMULK R0 R0 K0 [2]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // locals vs locals
    let result = compile_function_0("local a = 1 a /= a");
    let expected = "\nLOADN R0 1\nDIV R0 R0 R0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // locals vs expressions
    let result = compile_function_0("local a = 1 a /= a + 1");
    let expected = "\nLOADN R0 1\nADDK R1 R0 K0 [1]\nDIV R0 R0 R1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // upvalues
    let result = compile_function_0("local a = 1 function foo() a += 4 end");
    let expected = "\nGETUPVAL R0 0\nADDK R0 R0 K0 [4]\nSETUPVAL R0 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // table variants (indexed by string, number, variable)
    let result = compile_function_0("local a = {} a.foo += 5");
    let expected = "\nNEWTABLE R0 0 0\nGETTABLEKS R1 R0 K0 ['foo']\nADDK R1 R1 K1 [5]\nSETTABLEKS R1 R0 K0 ['foo']\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("local a = {} a[1] += 5");
    let expected =
        "\nNEWTABLE R0 0 0\nGETTABLEN R1 R0 1\nADDK R1 R1 K0 [5]\nSETTABLEN R1 R0 1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    let result = compile_function_0("local a = {} a[a] += 5");
    let expected =
        "\nNEWTABLE R0 0 0\nGETTABLE R1 R0 R0\nADDK R1 R1 K0 [5]\nSETTABLE R1 R0 R0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // left hand side is evaluated once
    let result = compile_function_0("foo()[bar()] += 5");
    let expected = "\nGETIMPORT R0 1 [foo]\nCALL R0 0 1\nGETIMPORT R1 3 [bar]\nCALL R1 0 1\nGETTABLE R2 R0 R1\nADDK R2 R2 K4 [5]\nSETTABLE R2 R0 R1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);
}
