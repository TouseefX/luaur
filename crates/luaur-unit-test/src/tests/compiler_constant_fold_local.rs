#[cfg(test)]
#[test]
fn compiler_constant_fold_local() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0("local a = 1 return a + a");
    let expected = "\nLOADN R0 2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a = 1 a = a + a return a");
    let expected = "\nLOADN R0 1\nADD R0 R0 R0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function("local a = 1 function foo() return a + a end", 0, 1, 0);
    let expected = "\nLOADN R0 2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function(
        "local a = 1 function foo() return a + a end function bar() a = 5 end",
        0,
        1,
        0,
    );
    let expected = "\nGETUPVAL R1 0\nGETUPVAL R2 0\nADD R0 R1 R2\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a return a");
    let expected = "\nLOADNIL R0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a, b = 1, 3 return a + 1, b");
    let expected = "\nLOADN R0 2\nLOADN R1 3\nRETURN R0 2\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a, b = 1 return a + 1, b");
    let expected = "\nLOADN R0 2\nLOADNIL R1\nRETURN R0 2\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a, b = ... return a + 1, b");
    let expected = "\nGETVARARGS R0 2\nADDK R2 R0 K0 [1]\nMOVE R3 R1\nRETURN R2 2\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local a, b = 1, ... return a + 1, b");
    let expected = "\nLOADN R0 1\nGETVARARGS R1 1\nLOADN R2 2\nMOVE R3 R1\nRETURN R2 2\n";
    assert_eq!(format!("\n{}", actual), expected);
}
