#[cfg(test)]
#[test]
fn compiler_no_builtin_fold_fenv() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"getfenv()

function test()
    return math.pi, math.sin(0)
end
"#,
        0,
        2,
        0,
    );

    let expected = "\nGETIMPORT R0 2 [math.pi]\nLOADN R2 0\nFASTCALL1 24 R2 L0\nGETIMPORT R1 4 [math.sin]\nCALL R1 1 1\nL0: RETURN R0 2\n";
    assert_eq!(format!("\n{}", actual), expected);
}
