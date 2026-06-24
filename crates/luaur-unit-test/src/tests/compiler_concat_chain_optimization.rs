#[cfg(test)]
#[test]
fn compiler_concat_chain_optimization() {
    use crate::functions::compile_function_0::compile_function_0;

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a, b = ...; return a .. b"),
        "\nGETVARARGS R0 2\nMOVE R3 R0\nMOVE R4 R1\nCONCAT R2 R3 R4\nRETURN R2 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a, b, c = ...; return a .. b .. c"),
        "\nGETVARARGS R0 3\nMOVE R4 R0\nMOVE R5 R1\nMOVE R6 R2\nCONCAT R3 R4 R6\nRETURN R3 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("local a, b, c = ...; return (a .. b) .. c"),
        "\nGETVARARGS R0 3\nMOVE R6 R0\nMOVE R7 R1\nCONCAT R4 R6 R7\nMOVE R5 R2\nCONCAT R3 R4 R5\nRETURN R3 1\n"
    );
}
