#[cfg(test)]
#[test]
fn compiler_assignment_global() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0("a = 2");
    let expected = "\nLOADN R0 2\nSETGLOBAL R0 K0 ['a']\nRETURN R0 0\n";

    assert_eq!(format!("\n{}", result), expected);
}
