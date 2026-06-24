#[cfg(test)]
#[test]
fn compiler_inline_upval() {
    use crate::functions::compile_function::compile_function;

    // if the argument is an upvalue, we naturally need to copy it to a local
    let actual = compile_function(
        r#"
local function foo(a)
    return a
end

local b = ...

function bar()
    local x = foo(b)
    return x
end
"#,
        1,
        2,
        0,
    );
    let expected = "\nGETUPVAL R1 0\nMOVE R0 R1\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // if the function uses an upvalue it's more complicated, because the lexical upvalue may become a local
    let actual = compile_function(
        r#"
local b = ...

local function foo(a)
    return a + b
end

local x = foo(42)
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nGETVARARGS R0 1\nDUPCLOSURE R1 K0 ['foo']\nCAPTURE VAL R0\nLOADN R3 42\nADD R2 R3 R0\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // sometimes the lexical upvalue is deep enough that it's still an upvalue though
    let actual = compile_function(
        r#"
local b = ...

function bar()
    local function foo(a)
        return a + b
    end

    local x = foo(42)
    return x
end
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nCAPTURE UPVAL U0\nLOADN R2 42\nGETUPVAL R3 0\nADD R1 R2 R3\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());
}
