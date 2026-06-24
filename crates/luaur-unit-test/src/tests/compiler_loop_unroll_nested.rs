#[cfg(test)]
#[test]
fn compiler_loop_unroll_nested() {
    use crate::functions::compile_function::compile_function;

    // we can unroll nested loops just fine
    let actual = "\n".to_string()
        + &compile_function(
            r#"local t = {}
for i=0,1 do
    for j=0,1 do
        t[i*2+(j+1)] = 0
    end
end
"#,
            0,
            2,
            0,
        );
    let expected = "\n\
NEWTABLE R0 0 0
LOADN R1 0
SETTABLEN R1 R0 1
LOADN R1 0
SETTABLEN R1 R0 2
LOADN R1 0
SETTABLEN R1 R0 3
LOADN R1 0
SETTABLEN R1 R0 4
RETURN R0 0
";
    assert_eq!(actual, expected);

    // if the inner loop is too expensive, we won't unroll the outer loop though, but we'll still unroll the inner loop!
    let actual2 = "\n".to_string()
        + &compile_function(
            r#"local t = {}
for i=0,3 do
    for j=0,3 do
        t[i*4+(j+1)] = 0
    end
end
"#,
            0,
            2,
            0,
        );
    let expected2 = "\n\
NEWTABLE R0 0 0
LOADN R3 0
LOADN R1 3
LOADN R2 1
FORNPREP R1 L1
L0: MULK R5 R3 K1 [4]
ADDK R4 R5 K0 [1]
LOADN R5 0
SETTABLE R5 R0 R4
MULK R5 R3 K1 [4]
ADDK R4 R5 K2 [2]
LOADN R5 0
SETTABLE R5 R0 R4
MULK R5 R3 K1 [4]
ADDK R4 R5 K3 [3]
LOADN R5 0
SETTABLE R5 R0 R4
MULK R5 R3 K1 [4]
ADDK R4 R5 K1 [4]
LOADN R5 0
SETTABLE R5 R0 R4
FORNLOOP R1 L0
L1: RETURN R0 0
";
    assert_eq!(actual2, expected2);

    // note, we sometimes can even unroll a loop with varying internal iterations
    let actual3 = "\n".to_string()
        + &compile_function(
            r#"local t = {}
for i=0,1 do
    for j=0,i do
        t[i*2+(j+1)] = 0
    end
end
"#,
            0,
            2,
            0,
        );
    let expected3 = "\n\
NEWTABLE R0 0 0
LOADN R1 0
SETTABLEN R1 R0 1
LOADN R1 0
SETTABLEN R1 R0 3
LOADN R1 0
SETTABLEN R1 R0 4
RETURN R0 0
";
    assert_eq!(actual3, expected3);
}
