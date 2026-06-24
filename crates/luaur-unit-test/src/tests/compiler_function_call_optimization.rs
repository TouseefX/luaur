#[cfg(test)]
#[test]
fn compiler_function_call_optimization() {
    use crate::functions::compile_function_0::compile_function_0;

    // direct call into local
    assert_eq!(
        "\n".to_string() + &compile_function_0("local foo = math.foo()"),
        "\nGETIMPORT R0 2 [math.foo]\nCALL R0 0 1\nRETURN R0 0\n"
    );

    // direct call into temp
    assert_eq!(
        "\n".to_string() + &compile_function_0("local foo = math.foo(math.bar())"),
        "\nGETIMPORT R0 2 [math.foo]\nGETIMPORT R1 4 [math.bar]\nCALL R1 0 -1\nCALL R0 -1 1\nRETURN R0 0\n"
    );

    // can't directly call into local since foo might be used as arguments of caller
    assert_eq!(
        "\n".to_string() + &compile_function_0("local foo foo = math.foo(foo)"),
        "\nLOADNIL R0\nGETIMPORT R1 2 [math.foo]\nMOVE R2 R0\nCALL R1 1 1\nMOVE R0 R1\nRETURN R0 0\n"
    );
}
