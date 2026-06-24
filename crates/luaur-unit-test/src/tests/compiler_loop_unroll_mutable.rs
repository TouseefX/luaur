#[cfg(test)]
#[test]
fn compiler_loop_unroll_mutable() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"for i=1,3 do
    i = 3
    print(i) -- should print 3 three times in a row
end
"#,
        0,
        2,
        0,
    );

    let expected = "\nLOADN R2 1\nLOADN R0 3\nLOADN R1 1\nFORNPREP R0 L1\nL0: MOVE R3 R2\nLOADN R3 3\nGETIMPORT R4 1 [print]\nMOVE R5 R3\nCALL R4 1 0\nFORNLOOP R0 L0\nL1: RETURN R0 0\n";

    assert_eq!("\n".to_string() + &actual, expected);
}
