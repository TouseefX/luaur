#[cfg(test)]
#[test]
fn compiler_constant_fold_string_len() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0("return #'string', #'', #'a', #('b')");
    let expected = "\nLOADN R0 6\nLOADN R1 0\nLOADN R2 1\nLOADN R3 1\nRETURN R0 4\n";

    assert_eq!(format!("\n{}", result), expected);
}
