#[cfg(test)]
#[test]
fn compiler_inline_iife() {
    use crate::functions::compile_function::compile_function;

    // IIFE with arguments
    let actual1 = compile_function(
        r#"function choose(a, b, c)
    return ((function(a, b, c) if a then return b else return c end end)(a, b, c))
end
"#,
        1,
        2,
        0,
    );
    let expected1 = "\nJUMPIFNOT R0 L0\nMOVE R3 R1\nRETURN R3 1\nL0: MOVE R3 R2\nRETURN R3 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    // IIFE with upvalues
    let actual2 = compile_function(
        r#"function choose(a, b, c)
    return ((function() if a then return b else return c end end)())
end
"#,
        1,
        2,
        0,
    );
    let expected2 = "\nJUMPIFNOT R0 L0\nMOVE R3 R1\nRETURN R3 1\nL0: MOVE R3 R2\nRETURN R3 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
