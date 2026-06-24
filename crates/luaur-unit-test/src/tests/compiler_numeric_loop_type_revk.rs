#[cfg(test)]
#[test]
fn compiler_numeric_loop_type_revk() {
    use crate::functions::compile_function::compile_function;

    let result = compile_function(
        r#"
for i = 1,10 do
    local a = i * 2
    local b = 3 * i
    local c = i + 2
    local d = 3 + i
    print(a, b, c, d)
end
"#,
        0,
        2,
        1,
    );

    let expected = r#"
LOADN R2 1
LOADN R0 10
LOADN R1 1
FORNPREP R0 L1
L0: MULK R3 R2 K0 [2]
MULK R4 R2 K1 [3]
ADDK R5 R2 K0 [2]
ADDK R6 R2 K1 [3]
GETIMPORT R7 3 [print]
MOVE R8 R3
MOVE R9 R4
MOVE R10 R5
MOVE R11 R6
CALL R7 4 0
FORNLOOP R0 L0
L1: RETURN R0 0
"#;

    assert_eq!("\n".to_string() + &result, expected);
}
