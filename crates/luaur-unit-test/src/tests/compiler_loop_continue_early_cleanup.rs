#[cfg(test)]
#[test]
fn compiler_loop_continue_early_cleanup() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"local y
repeat
    local a, b
    do continue end
    local c, d
    local function x()
        return a + b + c + d
    end

    c = 2
    a = 4

    y = x
until a"#,
        1,
        1,
        0,
    );
    let expected = "\nLOADNIL R0\nL0: LOADNIL R1\nLOADNIL R2\nJUMP L1\nLOADNIL R3\nLOADNIL R4\nNEWCLOSURE R5 P0\nCAPTURE REF R1\nCAPTURE REF R3\nLOADN R3 2\nLOADN R1 4\nMOVE R0 R5\nCLOSEUPVALS R3\nL1: JUMPIF R1 L2\nCLOSEUPVALS R1\nJUMPBACK L0\nL2: CLOSEUPVALS R1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
