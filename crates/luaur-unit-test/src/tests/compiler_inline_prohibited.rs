#[cfg(test)]
#[test]
fn compiler_inline_prohibited() {
    use crate::functions::compile_function::compile_function;

    // we can't inline variadic functions
    let actual1 = compile_function(
        r#"local function foo(...)
    return 42
end

local x = foo()
return x
"#,
        1,
        2,
        0,
    );
    let expected1 = "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nCALL R1 0 1\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    // we can't inline any functions in modules with getfenv/setfenv
    let actual2 = compile_function(
        r#"local function foo()
    return 42
end

local x = foo()
getfenv()
return x
"#,
        1,
        2,
        0,
    );
    let expected2 = "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nCALL R1 0 1\nGETIMPORT R2 2 [getfenv]\nCALL R2 0 0\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
