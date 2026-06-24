#[cfg(test)]
#[test]
fn compiler_custom_constant_fields() {
    use crate::functions::compile_function::compile_function;

    let result = compile_function(
        "return test.some_nil, test.some_boolean, test.some_number, test.some_string",
        0,
        2,
        0,
    );
    let expected =
        "\nLOADNIL R0\nLOADB R1 1\nLOADK R2 K0 [4.75]\nLOADK R3 K1 ['test']\nRETURN R0 4\n";

    assert_eq!("\n".to_string() + &result, expected);
}
