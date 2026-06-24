#[cfg(test)]
#[test]
fn compiler_vector_constants() {
    use crate::functions::compile_function::compile_function;

    let result1 = compile_function("return vector.create(1, 2)", 0, 2, 0);
    let expected1 = "\nLOADK R0 K0 [1, 2, 0]\nRETURN R0 1\n";
    assert_eq!("\n".to_string() + &result1, expected1);

    let result2 = compile_function("return vector.create(1, 2, 3)", 0, 2, 0);
    let expected2 = "\nLOADK R0 K0 [1, 2, 3]\nRETURN R0 1\n";
    assert_eq!("\n".to_string() + &result2, expected2);

    let result3 = compile_function("print(vector.create(1, 2, 3))", 0, 2, 0);
    let expected3 = "\nGETIMPORT R0 1 [print]\nLOADK R1 K2 [1, 2, 3]\nCALL R0 1 0\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &result3, expected3);

    let result4 = compile_function("print(vector.create(1, 2, 3, 4))", 0, 2, 0);
    let expected4 =
        "\nGETIMPORT R0 1 [print]\nLOADK R1 K2 [1, 2, 3, 4]\nCALL R0 1 0\nRETURN R0 0\n";
    assert_eq!("\n".to_string() + &result4, expected4);

    let result5 = compile_function(
        "return vector.create(0, 0, 0), vector.create(-0, 0, 0)",
        0,
        2,
        0,
    );
    let expected5 = "\nLOADK R0 K0 [0, 0, 0]\nLOADK R1 K1 [-0, 0, 0]\nRETURN R0 2\n";
    assert_eq!("\n".to_string() + &result5, expected5);

    let result6 = compile_function("return type(vector.create(0, 0, 0))", 0, 2, 0);
    let expected6 = "\nLOADK R0 K0 ['vector']\nRETURN R0 1\n";
    assert_eq!("\n".to_string() + &result6, expected6);

    let result7 = compile_function("return Vector3.new(1, 2, 3)", 0, 2, 0);
    let expected7 = "\nLOADK R0 K0 [1, 2, 3]\nRETURN R0 1\n";
    assert_eq!("\n".to_string() + &result7, expected7);
}
