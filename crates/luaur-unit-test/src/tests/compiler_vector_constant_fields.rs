#[cfg(test)]
#[test]
fn compiler_vector_constant_fields() {
    use crate::functions::compile_function::compile_function;

    assert_eq!(
        "\n".to_string() + &compile_function("return vector.one, vector.zero", 0, 2, 0),
        "\nLOADK R0 K0 [1, 1, 1]\nLOADK R1 K1 [0, 0, 0]\nRETURN R0 2\n"
    );

    assert_eq!(
        "\n".to_string() + &compile_function("return Vector3.one, Vector3.xAxis", 0, 2, 0),
        "\nLOADK R0 K0 [1, 1, 1]\nLOADK R1 K1 [1, 0, 0]\nRETURN R0 2\n"
    );

    assert_eq!(
        "\n".to_string()
            + &compile_function("return vector.one == vector.create(1, 1, 1)", 0, 2, 0),
        "\nLOADB R0 1\nRETURN R0 1\n"
    );
}
