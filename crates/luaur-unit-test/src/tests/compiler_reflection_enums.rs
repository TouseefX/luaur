#[cfg(test)]
#[test]
fn compiler_reflection_enums() {
    use crate::functions::compile_function_0::compile_function_0;

    assert_eq!(
        "\n".to_string() + &compile_function_0("return Enum.EasingStyle.Linear"),
        "\nGETIMPORT R0 3 [Enum.EasingStyle.Linear]\nRETURN R0 1\n"
    );
}
