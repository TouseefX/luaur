#[cfg(test)]
#[test]
fn compiler_interp_string_register_cleanup() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0(
        r#"
            local a, b, c = nil, "um", "uh oh"
            a = `foo{42}`
            print(a)
        "#,
    );

    let expected = "\n\
LOADNIL R0\n\
LOADK R1 K0 ['um']\n\
LOADK R2 K1 ['uh oh']\n\
LOADK R3 K2 ['foo%*']\n\
LOADN R5 42\n\
NAMECALL R3 R3 K3 ['format']\n\
CALL R3 2 1\n\
MOVE R0 R3\n\
GETIMPORT R3 5 [print]\n\
MOVE R4 R0\n\
CALL R3 1 0\n\
RETURN R0 0\n";

    assert_eq!(format!("\n{}", result), expected);
}
