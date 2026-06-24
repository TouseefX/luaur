#[cfg(test)]
#[test]
fn compiler_loop_continue() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0(
        "repeat if math.random() < 0.5 then continue else end break until false error()",
    );
    let expected = "\nL0: GETIMPORT R0 2 [math.random]\nCALL R0 0 1\nLOADK R1 K3 [0.5]\nJUMPIFNOTLT R0 R1 L2\nJUMP L1\nJUMP L2\nL1: JUMPBACK L0\nL2: GETIMPORT R0 5 [error]\nCALL R0 0 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0(
        "repeat if math.random() < 0.5 then continue end break until false error()",
    );
    let expected = "\nL0: GETIMPORT R0 2 [math.random]\nCALL R0 0 1\nLOADK R1 K3 [0.5]\nJUMPIFLT R0 R1 L1\nJUMP L2\nL1: JUMPBACK L0\nL2: GETIMPORT R0 5 [error]\nCALL R0 0 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
