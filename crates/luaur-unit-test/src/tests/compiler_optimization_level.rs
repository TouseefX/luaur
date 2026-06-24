#[cfg(test)]
#[test]
fn compiler_optimization_level() {
    use crate::functions::compile_function::compile_function;

    // at optimization level 1, no inlining is performed
    let actual1 = compile_function(
        r#"
local function foo(a)
    return a
end

return foo(42)
"#,
        1,
        1,
        0,
    );
    let expected1 =
        "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nLOADN R2 42\nCALL R1 1 -1\nRETURN R1 -1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    // you can override the level from 1 to 2 to force it
    let actual2 = compile_function(
        r#"--!optimize 2
local function foo(a)
    return a
end

return foo(42)
"#,
        1,
        1,
        0,
    );
    let expected2 = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 42\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    // you can also override it externally
    let actual3 = compile_function(
        r#"
local function foo(a)
    return a
end

return foo(42)
"#,
        1,
        2,
        0,
    );
    let expected3 = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 42\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual3), expected3);

    // ... after which you can downgrade it back via hot comment
    let actual4 = compile_function(
        r#"--!optimize 1
local function foo(a)
    return a
end

return foo(42)
"#,
        1,
        2,
        0,
    );
    let expected4 =
        "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nLOADN R2 42\nCALL R1 1 -1\nRETURN R1 -1\n";
    assert_eq!(format!("\n{}", actual4), expected4);
}
