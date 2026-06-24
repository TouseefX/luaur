#[cfg(test)]
#[test]
fn compiler_table_literals_number_index() {
    use crate::functions::compile_function_0::compile_function_0;

    // tables with [x] compile to SETTABLEN if the index is short
    let actual = "\n".to_string()
        + &compile_function_0("return {[2] = 2, [256] = 256, [0] = 0, [257] = 257}");
    let expected = "\n\
NEWTABLE R0 4 0
LOADN R1 2
SETTABLEN R1 R0 2
LOADN R1 256
SETTABLEN R1 R0 256
LOADN R1 0
LOADN R2 0
SETTABLE R2 R0 R1
LOADN R1 257
LOADN R2 257
SETTABLE R2 R0 R1
RETURN R0 1
";
    assert_eq!(actual, expected);

    // tables with [x] where x is sequential compile to correctly sized array + SETTABLEN
    let actual = "\n".to_string() + &compile_function_0("return {[1] = 1, [2] = 2}");
    let expected = "\n\
NEWTABLE R0 0 2
LOADN R1 1
SETTABLEN R1 R0 1
LOADN R1 2
SETTABLEN R1 R0 2
RETURN R0 1
";
    assert_eq!(actual, expected);

    // when index chain starts with 0, or isn't sequential, we disable the optimization
    let actual =
        "\n".to_string() + &compile_function_0("return {[0] = 0, [1] = 1, [2] = 2, [42] = 42}");
    let expected = "\n\
NEWTABLE R0 4 0
LOADN R1 0
LOADN R2 0
SETTABLE R2 R0 R1
LOADN R1 1
SETTABLEN R1 R0 1
LOADN R1 2
SETTABLEN R1 R0 2
LOADN R1 42
SETTABLEN R1 R0 42
RETURN R0 1
";
    assert_eq!(actual, expected);

    // we disable this optimization when the table has list elements for simplicity
    let actual = "\n".to_string() + &compile_function_0("return {[1] = 1, [2] = 2, 3}");
    let expected = "\n\
NEWTABLE R0 2 1
LOADN R2 1
SETTABLEN R2 R0 1
LOADN R2 2
SETTABLEN R2 R0 2
LOADN R1 3
SETLIST R0 R1 1 [1]
RETURN R0 1
";
    assert_eq!(actual, expected);

    // we can also correctly predict the array length for mixed tables
    let actual = "\n".to_string() + &compile_function_0("return {key = 1, value = 2, [1] = 42}");
    let expected = "\n\
NEWTABLE R0 2 1
LOADN R1 1
SETTABLEKS R1 R0 K0 ['key']
LOADN R1 2
SETTABLEKS R1 R0 K1 ['value']
LOADN R1 42
SETTABLEN R1 R0 1
RETURN R0 1
";
    assert_eq!(actual, expected);
}
