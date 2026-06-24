#[cfg(test)]
#[test]
fn compiler_reflection_bytecode() {
    use crate::functions::compile_function_0::compile_function_0;

    let source = r#"local part = Instance.new('Part', workspace)
part.Size = Vector3.new(1, 2, 3)
return part.Size.Z * part:GetMass()"#;

    let result = compile_function_0(source);
    let expected = "\nGETIMPORT R0 2 [Instance.new]\nLOADK R1 K3 ['Part']\nGETIMPORT R2 5 [workspace]\nCALL R0 2 1\nGETIMPORT R1 7 [Vector3.new]\nLOADN R2 1\nLOADN R3 2\nLOADN R4 3\nCALL R1 3 1\nSETTABLEKS R1 R0 K8 ['Size']\nGETTABLEKS R2 R0 K8 ['Size']\nGETTABLEKS R2 R2 K9 ['Z']\nNAMECALL R3 R0 K10 ['GetMass']\nCALL R3 1 1\nMUL R1 R2 R3\nRETURN R1 1\n";

    assert_eq!(format!("\n{}", result), expected);
}
