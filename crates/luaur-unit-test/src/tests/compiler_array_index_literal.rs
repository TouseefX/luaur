#[cfg(test)]
#[test]
fn compiler_array_index_literal() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = "\n".to_string()
        + &compile_function_0("local arr = {} return arr[0], arr[1], arr[256], arr[257]");
    let expected = "\n\
NEWTABLE R0 0 0
LOADN R2 0
GETTABLE R1 R0 R2
GETTABLEN R2 R0 1
GETTABLEN R3 R0 256
LOADN R5 257
GETTABLE R4 R0 R5
RETURN R1 4
";
    assert_eq!(actual, expected);

    let actual2 = "\n".to_string()
        + &compile_function_0(
            "local arr = {} local b = ... arr[0] = b arr[1] = b arr[256] = b arr[257] = b",
        );
    let expected2 = "\n\
NEWTABLE R0 0 1
GETVARARGS R1 1
LOADN R2 0
SETTABLE R1 R0 R2
SETTABLEN R1 R0 1
SETTABLEN R1 R0 256
LOADN R2 257
SETTABLE R1 R0 R2
RETURN R0 0
";
    assert_eq!(actual2, expected2);
}
