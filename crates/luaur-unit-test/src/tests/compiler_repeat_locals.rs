#[cfg(test)]
#[test]
fn compiler_repeat_locals() {
    use crate::functions::compile_function_0::compile_function_0;

    let result = compile_function_0("repeat local a a = 5 until a - 4 < 0 or a - 4 >= 0");
    let expected = "\nL0: LOADNIL R0\nLOADN R0 5\nSUBK R1 R0 K0 [4]\nLOADN R2 0\nJUMPIFLT R1 R2 L1\nSUBK R1 R0 K0 [4]\nLOADN R2 0\nJUMPIFLE R2 R1 L1\nJUMPBACK L0\nL1: RETURN R0 0\n";

    assert_eq!(format!("\n{}", result), expected);
}
