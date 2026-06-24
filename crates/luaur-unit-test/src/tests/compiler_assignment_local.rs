#[cfg(test)]
#[test]
fn compiler_assignment_local() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0("local a a = 2");
    let expected = "\nLOADNIL R0\nLOADN R0 2\nRETURN R0 0\n";

    assert_eq!(format!("\n{}", result), expected);
}
