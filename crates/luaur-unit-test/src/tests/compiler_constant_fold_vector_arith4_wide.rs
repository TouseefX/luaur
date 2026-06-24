#[cfg(test)]
#[test]
fn compiler_constant_fold_vector_arith4_wide() {
    use crate::functions::compile_function::compile_function;

    assert_eq!(
        "\n".to_string() + &compile_function("local n = 2; local a, b = vector.create(1, 2, 3, 4), vector.create(2, 4, 8, 1); return a + b", 0, 2, 0),
        "\nLOADK R0 K0 [3, 6, 11, 5]\nRETURN R0 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function("local n = 2; local a, b = vector.create(1, 2, 3, 4), vector.create(2, 4, 8, 1); return a - b", 0, 2, 0),
        "\nLOADK R0 K0 [-1, -2, -5, 3]\nRETURN R0 1\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function(
            "local n = 2; local a, b = vector.create(1, 2, 3, 4), vector.create(2, 4, 8, 1); return a * n, a * b, n * b, a * math.huge",
            0, 2, 0
        ),
        "\nLOADK R0 K0 [2, 4, 6, 8]\nLOADK R1 K1 [2, 8, 24, 4]\nLOADK R2 K2 [4, 8, 16, 2]\nLOADK R3 K3 [inf, inf, inf, inf]\nRETURN R0 4\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function(
            "local n = 2; local a, b = vector.create(1, 2, 3, 4), vector.create(2, 4, 8, 1); return a / n, a / b, n / b, a / math.huge",
            0, 2, 0
        ),
        "\nLOADK R0 K0 [0.5, 1, 1.5, 2]\nLOADK R1 K1 [0.5, 0.5, 0.375, 4]\nLOADK R2 K2 [1, 0.5, 0.25, 2]\nLOADK R3 K3 [0, 0, 0]\nRETURN R0 4\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function(
            "local n = 2; local a, b = vector.create(1, 2, 3, 4), vector.create(2, 4, 8, 1); return a // n, a // b, n // b",
            0, 2, 0
        ),
        "\nLOADK R0 K0 [0, 1, 1, 2]\nLOADK R1 K1 [0, 0, 0, 4]\nLOADK R2 K2 [1, 0, 0, 2]\nRETURN R0 3\n"
    );

    assert_eq!(
        "\n".to_string()
            + &compile_function("local a = vector.create(1, 2, 3, 4); return -a", 0, 2, 0),
        "\nLOADK R0 K0 [-1, -2, -3, -4]\nRETURN R0 1\n"
    );
}
