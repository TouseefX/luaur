#[cfg(test)]
#[test]
fn compiler_constant_fold_vector_components() {
    use crate::functions::compile_function::compile_function;

    let result1 = compile_function(
        r#"local a = vector.create(1, 2, 3, 4)
return a.x + a.y + a.z + a.w"#,
        0,
        2,
        0,
    );
    let expected1 = "\nLOADN R1 6\nLOADK R2 K0 [1, 2, 3, 4]\nGETTABLEKS R2 R2 K1 ['w']\nADD R0 R1 R2\nRETURN R0 1\n";
    assert_eq!("\n".to_string() + &result1, expected1);

    let result2 = compile_function(
        r#"local a = vector.create(1, 2, 3, 4)
return a.X + a.Y + a.Z + a.W"#,
        0,
        2,
        0,
    );
    let expected2 = "\nLOADN R1 6\nLOADK R2 K0 [1, 2, 3, 4]\nGETTABLEKS R2 R2 K1 ['W']\nADD R0 R1 R2\nRETURN R0 1\n";
    assert_eq!("\n".to_string() + &result2, expected2);
}
