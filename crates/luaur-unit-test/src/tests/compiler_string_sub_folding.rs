#[cfg(test)]
#[test]
fn compiler_string_sub_folding() {
    use crate::functions::compile_function::compile_function;

    let result = compile_function(
        r#"local s = "123456789"

return
    string.sub(s, 2, 4),
    string.sub(s, 7),
    string.sub(s, 7, 6),
    string.sub(s, 7, 7),
    string.sub(s, 0, 0),
    string.sub(s, -10, 10),
    string.sub(s, 1, 9),
    string.sub(s, -10, -20),
    string.sub(s, -1),
    string.sub(s, -4),
    string.sub(s, -6, -4)
"#,
        0,
        2,
        2,
    );

    let expected = r#"
LOADK R0 K0 ['234']
LOADK R1 K1 ['789']
LOADK R2 K2 ['']
LOADK R3 K3 ['7']
LOADK R4 K2 ['']
LOADK R5 K4 ['123456789']
LOADK R6 K4 ['123456789']
LOADK R7 K2 ['']
LOADK R8 K5 ['9']
LOADK R9 K6 ['6789']
LOADK R10 K7 ['456']
RETURN R0 11
"#;

    assert_eq!(format!("\n{}", result), expected);
}
