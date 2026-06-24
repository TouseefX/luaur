#[cfg(test)]
#[test]
fn compiler_unary_basic() {
    use crate::functions::compile_function_0::compile_function_0;

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a = ... return not a"),
        "\nGETVARARGS R0 1\nNOT R1 R0\nRETURN R1 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a = ... return -a"),
        "\nGETVARARGS R0 1\nMINUS R1 R0\nRETURN R1 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a = ... return #a"),
        "\nGETVARARGS R0 1\nLENGTH R1 R0\nRETURN R1 1\n"
    );
}
