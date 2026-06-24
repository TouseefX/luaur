#[cfg(test)]
#[test]
fn compiler_inline_basic() {
    use crate::functions::compile_function::compile_function;

    // inline function that returns a constant
    let actual = compile_function(
        r#"
local function foo()
    return 42
end

local x = foo()
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 42\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // inline function that returns the argument
    let actual = compile_function(
        r#"
local function foo(a)
    return a
end

local x = foo(42)
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADN R1 42\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // inline function that returns one of the two arguments
    let actual = compile_function(
        r#"
local function foo(a, b, c)
    if a then
        return b
    else
        return c
    end
end

local x = foo(true, math.random(), 5)
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETIMPORT R2 3 [math.random]\nCALL R2 0 1\nMOVE R1 R2\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // inline function that returns one of the two arguments
    let actual = compile_function(
        r#"
local function foo(a, b, c)
    if a then
        return b
    else
        return c
    end
end

local x = foo(true, 5, math.random())
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETIMPORT R2 3 [math.random]\nCALL R2 0 1\nLOADN R1 5\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());
}
