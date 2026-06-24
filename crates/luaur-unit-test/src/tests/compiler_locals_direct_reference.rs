#[cfg(test)]
#[test]
fn compiler_locals_direct_reference() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0("local a return a");
    let expected = "\nLOADNIL R0\nRETURN R0 1\n";

    assert_eq!(format!("\n{}", result), expected);
}
