#[cfg(test)]
#[test]
fn compiler_and_or_fold_left() {
    use crate::functions::compile_function_0::compile_function_0;

    let result1 = compile_function_0("local a = false return a and b");
    let expected1 = "\nLOADB R0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_function_0("local a = true return a or b");
    let expected2 = "\nLOADB R0 1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result2), expected2);

    let result3 = compile_function_0("local a = false return b and a");
    let expected3 = "\nGETIMPORT R1 2 [b]\nANDK R0 R1 K0 [false]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result3), expected3);

    let result4 = compile_function_0("local a = true return b or a");
    let expected4 = "\nGETIMPORT R1 2 [b]\nORK R0 R1 K0 [true]\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", result4), expected4);
}
