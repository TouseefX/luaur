#[cfg(test)]
#[test]
fn compiler_loop_unroll_nested_closure() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"for i=1,2 do
    local x = function() return i end
end
"#,
        1,
        2,
        0,
    );
    let expected = "\nLOADN R1 1\nNEWCLOSURE R0 P0\nCAPTURE VAL R1\nLOADN R1 2\nNEWCLOSURE R0 P0\nCAPTURE VAL R1\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
