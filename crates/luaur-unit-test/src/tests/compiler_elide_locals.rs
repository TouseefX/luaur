#[cfg(test)]
#[test]
fn compiler_elide_locals() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual1 = compile_function_0("local a, b = 1, 2\nreturn a + b\n");
    let expected1 = "\nLOADN R0 3\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function_0("local a = g()\nreturn a\n");
    let expected2 = "\nGETIMPORT R0 1 [g]\nCALL R0 0 1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    let actual3 = compile_function_0("local a = 1, g()\nreturn a\n");
    let expected3 = "\nLOADN R0 1\nGETIMPORT R1 1 [g]\nCALL R1 0 1\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual3), expected3);
}
