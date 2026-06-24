#[cfg(test)]
#[test]
fn compiler_inline_fast_call_k() {
    use crate::functions::compile_function::compile_function;

    let result = compile_function(
        r#"local function set(l0)
    rawset({}, l0)
end

set(false)
set({})
"#,
        1,
        2,
        0,
    );

    let expected = "\nDUPCLOSURE R0 K0 ['set']\nNEWTABLE R2 0 0\nFASTCALL2K 49 R2 K1 L0 [false]\nLOADK R3 K1 [false]\nGETIMPORT R1 3 [rawset]\nCALL R1 2 0\nL0: NEWTABLE R1 0 0\nNEWTABLE R3 0 0\nFASTCALL2 49 R3 R1 L1\nMOVE R4 R1\nGETIMPORT R2 3 [rawset]\nCALL R2 2 0\nL1: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &result, expected);
}
