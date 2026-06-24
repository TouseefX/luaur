#[cfg(test)]
#[test]
fn compiler_shared_closure() {
    use crate::functions::compile_function::compile_function;

    // closures can be shared even if functions refer to upvalues, as long as upvalues are top-level
    let actual = compile_function(
        r#"
local val = ...

local function foo()
    return function() return val end
end
"#,
        1,
        1,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 []\nCAPTURE UPVAL U0\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // ... as long as the values aren't mutated.
    let actual = compile_function(
        r#"
local val = ...

local function foo()
    return function() return val end
end

val = 5
"#,
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R0 P0\nCAPTURE UPVAL U0\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // making the upvalue non-toplevel disables the optimization since it's likely that it will change
    let actual = compile_function(
        r#"
local function foo(val)
    return function() return val end
end
"#,
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R1 P0\nCAPTURE VAL R0\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // the upvalue analysis is transitive through local functions, which allows for code reuse to not defeat the optimization
    let actual = compile_function(
        r#"
local val = ...

local function foo()
    local function bar()
        return val
    end

    return function() return bar() end
end
"#,
        2,
        1,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['bar']\nCAPTURE UPVAL U0\nDUPCLOSURE R1 K1 []\nCAPTURE VAL R0\nRETURN R1 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // as such, if the upvalue that we reach transitively isn't top-level we fall back to newclosure
    let actual = compile_function(
        r#"
local function foo(val)
    local function bar()
        return val
    end

    return function() return bar() end
end
"#,
        2,
        1,
        0,
    );
    let expected =
        "\nNEWCLOSURE R1 P0\nCAPTURE VAL R0\nNEWCLOSURE R2 P1\nCAPTURE VAL R1\nRETURN R2 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // we also allow recursive function captures to share the object, even when it's not top-level
    let actual = compile_function(
        "function test() local function foo() return foo() end end",
        1,
        1,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nCAPTURE VAL R0\nRETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());

    // multi-level recursive capture where function isn't top-level fails however.
    // note: this should probably be optimized to DUPCLOSURE but doing that requires a different upval tracking flow in the compiler
    let actual = compile_function(
        r#"
local function foo()
    local function bar()
        return function() return bar() end
    end
end
"#,
        1,
        1,
        0,
    );
    let expected = "\nNEWCLOSURE R0 P0\nCAPTURE UPVAL U0\nRETURN R0 1\n";
    assert_eq!(actual.trim(), expected.trim());

    // top level upvalues inside loops should not be shared -- note that the bytecode below only uses NEWCLOSURE
    let actual = compile_function(
        r#"
for i=1,10 do
    print(function() return i end)
end

for k,v in pairs(...) do
    print(function() return k end)
end

for i=1,10 do
    local j = i
    print(function() return j end)
end
"#,
        3,
        1,
        0,
    );
    let expected = "\nLOADN R2 1\nLOADN R0 10\nLOADN R1 1\nFORNPREP R0 L1\nL0: GETIMPORT R3 1 [print]\nNEWCLOSURE R4 P0\nCAPTURE VAL R2\nCALL R3 1 0\nFORNLOOP R0 L0\nL1: GETIMPORT R0 3 [pairs]\nGETVARARGS R1 -1\nCALL R0 -1 3\nFORGPREP_NEXT R0 L3\nL2: GETIMPORT R5 1 [print]\nNEWCLOSURE R6 P1\nCAPTURE VAL R3\nCALL R5 1 0\nL3: FORGLOOP R0 L2 2\nLOADN R2 1\nLOADN R0 10\nLOADN R1 1\nFORNPREP R0 L5\nL4: GETIMPORT R3 1 [print]\nNEWCLOSURE R4 P2\nCAPTURE VAL R2\nCALL R3 1 0\nFORNLOOP R0 L4\nL5: RETURN R0 0\n";
    assert_eq!(actual.trim(), expected.trim());
}
