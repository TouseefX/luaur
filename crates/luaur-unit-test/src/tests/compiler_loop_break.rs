#[cfg(test)]
#[test]
fn compiler_loop_break() {
    use crate::functions::compile_function_0::compile_function_0;

    // default codegen: compile breaks as unconditional jumps
    let actual1 =
        compile_function_0("while true do if math.random() < 0.5 then break else end end");
    let expected1 = "\nL0: GETIMPORT R0 2 [math.random]\nCALL R0 0 1\nLOADK R1 K3 [0.5]\nJUMPIFNOTLT R0 R1 L1\nRETURN R0 0\nL1: JUMPBACK L0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    // optimization: if then body is a break statement, flip the branches
    let actual2 = compile_function_0("while true do if math.random() < 0.5 then break end end");
    let expected2 = "\nL0: GETIMPORT R0 2 [math.random]\nCALL R0 0 1\nLOADK R1 K3 [0.5]\nJUMPIFLT R0 R1 L1\nJUMPBACK L0\nL1: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
