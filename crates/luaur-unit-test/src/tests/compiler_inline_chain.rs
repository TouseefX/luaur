#[cfg(test)]
#[test]
fn compiler_inline_chain() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"local function foo(a, b)
    return a + b
end

local function bar(x)
    return foo(x, 1) * foo(x, -1)
end

local function baz()
    return (bar(42))
end

return (baz())
"#,
        3,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nDUPCLOSURE R1 K1 ['bar']\nDUPCLOSURE R2 K2 ['baz']\nLOADN R4 43\nLOADN R5 41\nMUL R3 R4 R5\nRETURN R3 1\n";
    assert_eq!(format!("\n{}", actual), expected);
}
