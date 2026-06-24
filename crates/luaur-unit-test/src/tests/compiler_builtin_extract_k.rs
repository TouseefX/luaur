#[cfg(test)]
#[test]
fn compiler_builtin_extract_k() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0(
        r#"local v = ...

return bit32.extract(v, 1, 3)
"#,
    );

    let expected = "\nGETVARARGS R0 1\nFASTCALL2K 59 R0 K0 L0 [65]\nMOVE R2 R0\nLOADK R3 K1 [1]\nLOADK R4 K2 [3]\nGETIMPORT R1 5 [bit32.extract]\nCALL R1 3 -1\nL0: RETURN R1 -1\n";
    assert_eq!("\n".to_string() + &result, expected);
}
