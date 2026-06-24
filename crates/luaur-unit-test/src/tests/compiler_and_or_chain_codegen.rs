#[cfg(test)]
#[test]
fn compiler_and_or_chain_codegen() {
    use crate::functions::compile_function_0::compile_function_0;

    let source = r#"
    return
        (1 - verticalGradientTurbulence < waterLevel + .015 and Enum.Material.Sand)
        or (sandbank>0 and sandbank<1 and Enum.Material.Sand)--this for canyonbase sandbanks
        or Enum.Material.Sandstone
    "#;

    let expected = r#"
GETIMPORT R2 2 [verticalGradientTurbulence]
SUBRK R1 K0 [1] R2
GETIMPORT R3 5 [waterLevel]
ADDK R2 R3 K3 [0.014999999999999999]
JUMPIFNOTLT R1 R2 L0
GETIMPORT R0 9 [Enum.Material.Sand]
JUMPIF R0 L2
L0: GETIMPORT R1 11 [sandbank]
LOADN R2 0
JUMPIFNOTLT R2 R1 L1
GETIMPORT R1 11 [sandbank]
LOADN R2 1
JUMPIFNOTLT R1 R2 L1
GETIMPORT R0 9 [Enum.Material.Sand]
JUMPIF R0 L2
L1: GETIMPORT R0 13 [Enum.Material.Sandstone]
L2: RETURN R0 1
"#;

    assert_eq!("\n".to_string() + &compile_function_0(source), expected);
}
