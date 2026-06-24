#[cfg(test)]
#[test]
fn compiler_skip_self_assignment() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a a = a"),
        "\nLOADNIL R0\nRETURN R0 0\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a a = a :: number"),
        "\nLOADNIL R0\nRETURN R0 0\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a a = (((a)))"),
        "\nLOADNIL R0\nRETURN R0 0\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function("local a a = a", 0, 0, 0),
        "\nLOADNIL R0\nMOVE R0 R0\nRETURN R0 0\n"
    );
}
