#[cfg(test)]
#[test]
fn compiler_string_char_folding() {
    use crate::functions::compile_function::compile_function;

    let result = compile_function(
        r#"local s1 = string.char(49, 50, 51, 52, 53, 54)
local s2 = string.char()
local s3 = string.char(0, 0, 0)
local s4 = string.char(49, 50, 0, 52, 53, 0)
return s1, s2, s3, s4
"#,
        0,
        2,
        0,
    );

    let expected = "\nLOADK R0 K0 ['123456']\nLOADK R1 K1 ['']\nLOADK R2 K2 ['\\x00\\x00\\x00']\nLOADK R3 K3 ['12\\x0045\\x00']\nRETURN R0 4\n";

    assert_eq!("\n".to_string() + &result, expected);
}
