#[cfg(test)]
#[test]
fn compiler_fast_call_upvalue_fallback() {
    use crate::functions::compile_function::compile_function;

    let result = compile_function(
        r#"local string = string

local function foo(t)
    return string.char(table.unpack(t))
end
"#,
        0,
        2,
        0,
    );

    let expected = "\nFASTCALL1 53 R0 L0\nMOVE R3 R0\nGETIMPORT R2 2 [table.unpack]\nCALL R2 1 -1\nL0: FASTCALL 42 L1\nGETUPVAL R1 0\nGETTABLEKS R1 R1 K3 ['char']\nCALL R1 -1 1\nL1: RETURN R1 1\n";
    assert_eq!("\n".to_string() + &result, expected);
}
