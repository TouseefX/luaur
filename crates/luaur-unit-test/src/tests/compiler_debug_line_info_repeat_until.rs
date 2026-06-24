#[cfg(test)]
#[test]
fn compiler_debug_line_info_repeat_until() {
    use crate::functions::compile_function_0_coverage::compile_function_0_coverage;

    let actual = compile_function_0_coverage(
        r#"
local f = 0
repeat
    f += 1
    if f == 1 then
        print(f)
    else
        f = 0
    end
until f == 0
"#,
        0,
    );
    let expected = "\n2: LOADN R0 0\n4: L0: ADDK R0 R0 K0 [1]\n5: JUMPXEQKN R0 K0 L1 NOT [1]\n6: GETIMPORT R1 2 [print]\n6: MOVE R2 R0\n6: CALL R1 1 0\n6: JUMP L2\n8: L1: LOADN R0 0\n10: L2: JUMPXEQKN R0 K3 L3 [0]\n10: JUMPBACK L0\n11: L3: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
