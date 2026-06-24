#[cfg(test)]
#[test]
fn compiler_inline_multret() {
    use crate::functions::compile_function::compile_function;

    // inlining a function in multret context is prohibited since we can't adjust L->top outside of CALL/GETVARARGS
    let actual = compile_function(
        r#"
local function foo(a)
    return a()
end

return foo(42)
"#,
        1,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
MOVE R1 R0
LOADN R2 42
CALL R1 1 -1
RETURN R1 -1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // however, if we can deduce statically that a function always returns a single value, the inlining will work
    let actual = compile_function(
        r#"
local function foo(a)
    return a
end

return foo(42)
"#,
        1,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
LOADN R1 42
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // this analysis will also propagate through other functions
    let actual = compile_function(
        r#"
local function foo(a)
    return a
end

local function bar(a)
    return foo(a)
end

return bar(42)
"#,
        2,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
DUPCLOSURE R1 K1 ['bar']
LOADN R2 42
RETURN R2 1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // we currently don't do this analysis fully for recursive functions since they can't be inlined anyway
    let actual = compile_function(
        r#"
local function foo(a)
    return foo(a)
end

return foo(42)
"#,
        1,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
CAPTURE VAL R0
MOVE R1 R0
LOADN R2 42
CALL R1 1 -1
RETURN R1 -1
"#;
    assert_eq!(actual.trim(), expected.trim());

    // we do this for builtins though as we assume getfenv is not used or is not changing arity
    let actual = compile_function(
        r#"
local function foo(a)
    return math.abs(a)
end

return foo(42)
"#,
        1,
        2,
        0,
    );
    let expected = r#"
DUPCLOSURE R0 K0 ['foo']
LOADN R1 42
RETURN R1 1
"#;
    assert_eq!(actual.trim(), expected.trim());
}
