#[cfg(test)]
#[test]
fn compiler_inline_loop_iteration() {
    use crate::functions::compile_function::compile_function;

    let actual = compile_function(
        r#"local function foo(a)
    local s = 0
    for i = 1,a do
        s += i
    end
    return s
end

local x = foo(3)
local y = foo(100)
return x, y
"#,
        1,
        2,
        0,
    );

    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R2 0\nADDK R2 R2 K1 [1]\nADDK R2 R2 K2 [2]\nADDK R2 R2 K3 [3]\nMOVE R1 R2\nMOVE R2 R0\nLOADN R3 100\nCALL R2 1 1\nRETURN R1 2\n";

    assert_eq!("\n".to_string() + &actual, expected);
}
