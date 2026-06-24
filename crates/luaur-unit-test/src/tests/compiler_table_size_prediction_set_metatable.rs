#[cfg(test)]
#[test]
fn compiler_table_size_prediction_set_metatable() {
    use crate::functions::compile_function_0::compile_function_0;

    let source = "local t = setmetatable({}, nil)\nt.field1 = 1\nt.field2 = 2\nreturn t";
    let actual = "\n".to_string() + &compile_function_0(source);
    let expected = "\n\
NEWTABLE R1 2 0
FASTCALL2K 61 R1 K0 L0 [nil]
LOADK R2 K0 [nil]
GETIMPORT R0 2 [setmetatable]
CALL R0 2 1
L0: LOADN R1 1
SETTABLEKS R1 R0 K3 ['field1']
LOADN R1 2
SETTABLEKS R1 R0 K4 ['field2']
RETURN R0 1
";
    assert_eq!(actual, expected);
}
