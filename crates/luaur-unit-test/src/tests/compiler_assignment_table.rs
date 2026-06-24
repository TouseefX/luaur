#[cfg(test)]
#[test]
fn compiler_assignment_table() {
    use crate::functions::compile_function_0::compile_function_0;

    let source = "local c = ... local a = {} a.b = 2 a.b = c";

    let result = compile_function_0(source);
    let expected = "\nGETVARARGS R0 1\nNEWTABLE R1 1 0\nLOADN R2 2\nSETTABLEKS R2 R1 K0 ['b']\nSETTABLEKS R0 R1 K0 ['b']\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", result), expected);
}
