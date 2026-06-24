#[cfg(test)]
#[test]
fn compiler_loop_unroll_unsupported() {
    use crate::functions::compile_function::compile_function;

    // can't unroll loops with non-constant bounds
    let result1 = compile_function(
        r#"for i=x,y,z do
end
"#,
        0,
        2,
        0,
    );
    let expected1 = "\nGETIMPORT R2 1 [x]\nGETIMPORT R0 3 [y]\nGETIMPORT R1 5 [z]\nFORNPREP R0 L1\nL0: FORNLOOP R0 L0\nL1: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &result1, expected1);

    // can't unroll loops with bounds where we can't compute trip count
    let result2 = compile_function(
        r#"for i=1,1,0 do
end
"#,
        0,
        2,
        0,
    );
    let expected2 = "\nLOADN R2 1\nLOADN R0 1\nLOADN R1 0\nFORNPREP R0 L1\nL0: FORNLOOP R0 L0\nL1: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &result2, expected2);

    // can't unroll loops with bounds that might be imprecise (non-integer)
    let result3 = compile_function(
        r#"for i=1,2,0.1 do
end
"#,
        0,
        2,
        0,
    );
    let expected3 = "\nLOADN R2 1\nLOADN R0 2\nLOADK R1 K0 [0.10000000000000001]\nFORNPREP R0 L1\nL0: FORNLOOP R0 L0\nL1: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &result3, expected3);

    // can't unroll loops if the bounds are too large, as it might overflow trip count math
    let result4 = compile_function(
        r#"for i=4294967295,4294967296 do
end
"#,
        0,
        2,
        0,
    );
    let expected4 = "\nLOADK R2 K0 [4294967295]\nLOADK R0 K1 [4294967296]\nLOADN R1 1\nFORNPREP R0 L1\nL0: FORNLOOP R0 L0\nL1: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &result4, expected4);
}
