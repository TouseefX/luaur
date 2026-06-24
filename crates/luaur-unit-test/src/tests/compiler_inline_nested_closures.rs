#[cfg(test)]
#[test]
fn compiler_inline_nested_closures() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"
local function foo(x)
    return function(y) return x + y end
end

local x = foo(1)(2)
return x
"#,
        2,
        2,
        0,
    );

    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R2 1\nNEWCLOSURE R1 P1\nCAPTURE VAL R2\nLOADN R2 2\nCALL R1 1 1\nRETURN R1 1\n";

    assert_eq!(format!("\n{}", actual), expected);
}
