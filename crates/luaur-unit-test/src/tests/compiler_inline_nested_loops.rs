#[cfg(test)]
#[test]
fn compiler_inline_nested_loops() {
    use crate::functions::compile_function::compile_function;

    // functions with basic loops get inlined
    let actual = "\n".to_string()
        + &compile_function(
            r#"local function foo(t)
    for i=1,3 do
        t[i] = i
    end
    return t
end

local x = foo({})
return x
"#,
            1,
            2,
            0,
        );
    let expected = "\n\
DUPCLOSURE R0 K0 ['foo']
NEWTABLE R2 0 0
LOADN R3 1
SETTABLEN R3 R2 1
LOADN R3 2
SETTABLEN R3 R2 2
LOADN R3 3
SETTABLEN R3 R2 3
MOVE R1 R2
RETURN R1 1
";
    assert_eq!(actual, expected);

    // we can even unroll the loops based on inline argument
    let actual2 = "\n".to_string()
        + &compile_function(
            r#"local function foo(t, n)
    for i=1, n do
        t[i] = i
    end
    return t
end

local x = foo({}, 3)
return x
"#,
            1,
            2,
            0,
        );
    let expected2 = "\n\
DUPCLOSURE R0 K0 ['foo']
NEWTABLE R2 0 0
LOADN R3 1
SETTABLEN R3 R2 1
LOADN R3 2
SETTABLEN R3 R2 2
LOADN R3 3
SETTABLEN R3 R2 3
MOVE R1 R2
RETURN R1 1
";
    assert_eq!(actual2, expected2);
}
