#[cfg(test)]
#[test]
fn compiler_inline_capture() {
    use crate::functions::compile_function::compile_function;

    // if the argument is captured by a nested closure, normally we can rely on capture by value
    let actual = compile_function(
        r#"
local function foo(a)
    return function() return a end
end

local x = ...
local y = foo(x)
return y
"#,
        2,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
GETVARARGS R1 1
NEWCLOSURE R2 P1
CAPTURE VAL R1
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // if the argument is a constant, we move it to a register so that capture by value can happen
    let actual = compile_function(
        r#"
local function foo(a)
    return function() return a end
end

local y = foo(42)
return y
"#,
        2,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
LOADN R2 42
NEWCLOSURE R1 P1
CAPTURE VAL R2
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // if the argument is an externally mutated variable, we copy it to an argument and capture it by value
    let actual = compile_function(
        r#"
local function foo(a)
    return function() return a end
end

local x x = 42
local y = foo(x)
return y
"#,
        2,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
LOADNIL R1
LOADN R1 42
MOVE R3 R1
NEWCLOSURE R2 P1
CAPTURE VAL R3
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // finally, if the argument is mutated internally, we must capture it by reference and close the upvalue
    let actual = compile_function(
        r#"
local function foo(a)
    a = a or 42
    return function() return a end
end

local y = foo()
return y
"#,
        2,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
LOADNIL R2
ORK R2 R2 K1 [42]
NEWCLOSURE R1 P1
CAPTURE REF R2
CLOSEUPVALS R2
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // note that capture might need to be performed during the fallthrough block
    let actual = compile_function(
        r#"
local function foo(a)
    a = a or 42
    print(function() return a end)
end

local x = ...
local y = foo(x)
return y
"#,
        2,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
GETVARARGS R1 1
MOVE R3 R1
ORK R3 R3 K1 [42]
GETIMPORT R4 3 [print]
NEWCLOSURE R5 P1
CAPTURE REF R3
CALL R4 1 0
LOADNIL R2
CLOSEUPVALS R3
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // note that mutation and capture might be inside internal control flow
    // TODO: this has an oddly redundant CLOSEUPVALS after JUMP; it's not due to inlining, and is an artifact of how StatBlock/StatReturn interact
    // fixing this would reduce the number of redundant CLOSEUPVALS a bit but it only affects bytecode size as these instructions aren't executed
    let actual = compile_function(
        r#"
local function foo(a)
    if not a then
        local b b = 42
        return function() return b end
    end
end

local x = ...
local y = foo(x)
return y, x
"#,
        2,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
GETVARARGS R1 1
JUMPIF R1 L0
LOADNIL R3
LOADN R3 42
NEWCLOSURE R2 P1
CAPTURE REF R3
CLOSEUPVALS R3
JUMP L1
CLOSEUPVALS R3
L0: LOADNIL R2
L1: MOVE R3 R2
MOVE R4 R1
RETURN R3 2
"#;
    assert_eq!(actual.trim(), expected.trim());
}
