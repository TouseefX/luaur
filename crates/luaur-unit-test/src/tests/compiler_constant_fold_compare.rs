#[cfg(test)]
#[test]
fn compiler_constant_fold_compare() {
    use crate::functions::compile_function_0::compile_function_0;

    assert_eq!(
        "\n".to_string() + &compile_function_0("return 1 < 1, 1 < 2"),
        "\nLOADB R0 0\nLOADB R1 1\nRETURN R0 2\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("return 1 <= 1, 1 <= 2"),
        "\nLOADB R0 1\nLOADB R1 1\nRETURN R0 2\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("return 1 > 1, 1 > 2"),
        "\nLOADB R0 0\nLOADB R1 0\nRETURN R0 2\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("return 1 >= 1, 1 >= 2"),
        "\nLOADB R0 1\nLOADB R1 0\nRETURN R0 2\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("return nil == 1, nil ~= 1, nil == nil, nil ~= nil"),
        "\nLOADB R0 0\nLOADB R1 1\nLOADB R2 1\nLOADB R3 0\nRETURN R0 4\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function_0("return 2 == 1, 2 ~= 1, 1 == 1, 1 ~= 1"),
        "\nLOADB R0 0\nLOADB R1 1\nLOADB R2 1\nLOADB R3 0\nRETURN R0 4\n"
    );

    assert_eq!(
        "\n".to_string()
            + &compile_function_0(
                "return true == false, true ~= false, true == true, true ~= true"
            ),
        "\nLOADB R0 0\nLOADB R1 1\nLOADB R2 1\nLOADB R3 0\nRETURN R0 4\n"
    );

    assert_eq!(
        "\n".to_string()
            + &compile_function_0("return 'a' == 'b', 'a' ~= 'b', 'a' == 'a', 'a' ~= 'a'"),
        "\nLOADB R0 0\nLOADB R1 1\nLOADB R2 1\nLOADB R3 0\nRETURN R0 4\n"
    );
}
