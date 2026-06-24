#[cfg(test)]
#[test]
fn compiler_inline_arg_mismatch() {
    use crate::functions::compile_function::compile_function;

    // caller might not have enough arguments
    let actual = compile_function(
        r#"local function foo(a)
    return a
end

local x = foo()
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADNIL R1\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &actual, expected);

    // caller might be using multret for arguments
    let actual = compile_function(
        r#"local function foo(a, b)
    return a + b
end

local x = foo(math.modf(1.5))
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADK R3 K1 [1.5]\nFASTCALL1 20 R3 L0\nGETIMPORT R2 4 [math.modf]\nCALL R2 1 2\nL0: ADD R1 R2 R3\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &actual, expected);

    // caller might be using varargs for arguments
    let actual = compile_function(
        r#"local function foo(a, b)
    return a + b
end

local x = foo(...)
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETVARARGS R2 2\nADD R1 R2 R3\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &actual, expected);

    // caller might have too many arguments, but we still need to compute them for side effects
    let actual = compile_function(
        r#"local function foo(a)
    return a
end

local x = foo(42, print())
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nGETIMPORT R2 2 [print]\nCALL R2 0 1\nLOADN R1 42\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &actual, expected);

    // caller might not have enough arguments, and the arg might be mutated so it needs a register
    let actual = compile_function(
        r#"local function foo(a)
    a = 42
    return a
end

local x = foo()
return x
"#,
        1,
        2,
        0,
    );
    let expected = "\nDUPCLOSURE R0 K0 ['foo']\nLOADNIL R2\nLOADN R2 42\nMOVE R1 R2\nRETURN R1 1\n";
    assert_eq!("\n".to_string() + &actual, expected);
}
