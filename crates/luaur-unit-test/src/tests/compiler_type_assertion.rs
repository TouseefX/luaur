#[cfg(test)]
#[test]
fn compiler_type_assertion() {
    use crate::functions::compile_function_0::compile_function_0;

    // validate that type assertions work with the compiler and that the code inside type assertion isn't evaluated
    let actual = compile_function_0("print(foo() :: typeof(error(\"compile time\")))");
    let expected =
        "\nGETIMPORT R0 1 [print]\nGETIMPORT R1 3 [foo]\nCALL R1 0 1\nCALL R0 1 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // note that above, foo() is treated as single-arg function; removing type assertion changes the bytecode
    let actual = compile_function_0("print(foo())");
    let expected =
        "\nGETIMPORT R0 1 [print]\nGETIMPORT R1 3 [foo]\nCALL R1 0 -1\nCALL R0 -1 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
