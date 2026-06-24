#[cfg(test)]
#[test]
fn compiler_assignment_conflict() {
    use crate::functions::compile_function_0::compile_function_0;

    // assignments are left to right
    let result = compile_function_0("local a, b a, b = 1, 2");
    let expected = "\nLOADNIL R0\nLOADNIL R1\nLOADN R0 1\nLOADN R1 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // if assignment of a local invalidates a direct register reference in later assignments, the value is assigned to a temp register first
    let result = compile_function_0("local a a, a[1] = 1, 2");
    let expected =
        "\nLOADNIL R0\nLOADN R1 1\nLOADN R2 2\nSETTABLEN R2 R0 1\nMOVE R0 R1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // note that this doesn't happen if the local assignment happens last naturally
    let result = compile_function_0("local a a[1], a = 1, 2");
    let expected =
        "\nLOADNIL R0\nLOADN R2 1\nLOADN R1 2\nSETTABLEN R2 R0 1\nMOVE R0 R1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // this will happen if assigned register is used in any table expression, including as an object...
    let result = compile_function_0("local a a, a.foo = 1, 2");
    let expected = "\nLOADNIL R0\nLOADN R1 1\nLOADN R2 2\nSETTABLEKS R2 R0 K0 ['foo']\nMOVE R0 R1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // ... or a table index ...
    let result = compile_function_0("local a a, foo[a] = 1, 2");
    let expected = "\nLOADNIL R0\nGETIMPORT R1 1 [foo]\nLOADN R2 1\nLOADN R3 2\nSETTABLE R3 R1 R0\nMOVE R0 R2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // ... or both ...
    let result = compile_function_0("local a a, a[a] = 1, 2");
    let expected =
        "\nLOADNIL R0\nLOADN R1 1\nLOADN R2 2\nSETTABLE R2 R0 R0\nMOVE R0 R1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // ... or both with two different locals ...
    let result = compile_function_0("local a, b a, b, a[b] = 1, 2, 3");
    let expected = "\nLOADNIL R0\nLOADNIL R1\nLOADN R2 1\nLOADN R3 2\nLOADN R4 3\nSETTABLE R4 R0 R1\nMOVE R0 R2\nMOVE R1 R3\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);

    // however note that if it participates in an expression on the left hand side, there's no point reassigning it since we'd compute the expr value
    // into a temp register
    let result = compile_function_0("local a a, foo[a + 1] = 1, 2");
    let expected = "\nLOADNIL R0\nGETIMPORT R1 1 [foo]\nADDK R2 R0 K2 [1]\nLOADN R0 1\nLOADN R3 2\nSETTABLE R3 R1 R2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);
}
