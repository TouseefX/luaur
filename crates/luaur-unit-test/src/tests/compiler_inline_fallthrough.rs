#[cfg(test)]
#[test]
fn compiler_inline_fallthrough() {
    use crate::functions::compile_function::compile_function;

    // if the function doesn't return, we still fill the results with nil
    let actual = compile_function(
        r#"
local function foo()
end

local a, b = foo()
return a, b
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADNIL R1\nLOADNIL R2\nRETURN R1 2\n";
    assert_eq!(actual.trim(), expected.trim());

    // this happens even if the function returns conditionally
    let actual = compile_function(
        r#"
local function foo(a)
    if a then return 42 end
end

local a, b = foo(false)
return a, b
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADNIL R1\nLOADNIL R2\nRETURN R1 2\n";
    assert_eq!(actual.trim(), expected.trim());

    // note though that we can't inline a function like this in multret context
    // this is because we don't have a SETTOP instruction
    let actual = compile_function(
        r#"
local function foo()
end

return foo()
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nCALL R1 0 -1\nRETURN R1 -1\n";
    assert_eq!(actual.trim(), expected.trim());
}
