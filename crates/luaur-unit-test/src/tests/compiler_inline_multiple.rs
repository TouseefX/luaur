#[cfg(test)]
#[test]
fn compiler_inline_multiple() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"local function foo(a, b)
    return a + b
end

local x, y = ...
local a = foo(x, 1)
local b = foo(1, x)
local c = foo(1, 2)
local d = foo(x, y)
return a, b, c, d
"#,
        1,
        2,
        0,
    );

    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETVARARGS R1 2\nADDK R3 R1 K1 [1]\nLOADN R5 1\nADD R4 R5 R1\nLOADN R5 3\nADD R6 R1 R2\nRETURN R3 4\n";

    assert_eq!(format!("\n{}", actual), expected);
}
