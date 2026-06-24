#[cfg(test)]
#[test]
fn compiler_table_size_prediction_loop() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = "\n".to_string()
        + &compile_function_0("local t = {}\nfor i=1,4 do\n    t[i] = 0\nend\nreturn t");
    let expected = "\n\
NEWTABLE R0 0 4
LOADN R3 1
LOADN R1 4
LOADN R2 1
FORNPREP R1 L1
L0: LOADN R4 0
SETTABLE R4 R0 R3
FORNLOOP R1 L0
L1: RETURN R0 1
";
    assert_eq!(actual, expected);
}
