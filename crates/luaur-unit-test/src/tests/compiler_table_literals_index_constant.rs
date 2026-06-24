#[cfg(test)]
#[test]
fn compiler_table_literals_index_constant() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = "\n".to_string()
        + &compile_function_0("local a, b = \"key\", \"value\"\nreturn {[a] = 42, [b] = 0}");
    let expected = "\n\
NEWTABLE R0 2 0
LOADN R1 42
SETTABLEKS R1 R0 K0 ['key']
LOADN R1 0
SETTABLEKS R1 R0 K1 ['value']
RETURN R0 1
";
    assert_eq!(actual, expected);

    let actual2 =
        "\n".to_string() + &compile_function_0("local a, b = 1, 2\nreturn {[a] = 42, [b] = 0}");
    let expected2 = "\n\
NEWTABLE R0 0 2
LOADN R1 42
SETTABLEN R1 R0 1
LOADN R1 0
SETTABLEN R1 R0 2
RETURN R0 1
";
    assert_eq!(actual2, expected2);
}
