#[cfg(test)]
#[test]
fn compiler_inline_thresholds() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FInt;

    let _sfis = [
        ScopedFastInt::new(&FInt::LuauCompileInlineThreshold, 25),
        ScopedFastInt::new(&FInt::LuauCompileInlineThresholdMaxBoost, 300),
        ScopedFastInt::new(&FInt::LuauCompileInlineDepth, 2),
    ];

    // this function has enormous register pressure (50 regs) so we choose not to inline it
    let actual1 = compile_function(
        r#"
local function foo()
    return {{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{{}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}}
end

return (foo())
"#,
        1,
        2,
        0,
    );
    let expected1 = "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nCALL R1 0 1\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    // this function has less register pressure but a large cost
    let actual2 = compile_function(
        r#"
local function foo()
    return {},{},{},{},{}
end

return (foo())
"#,
        1,
        2,
        0,
    );
    let expected2 = "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nCALL R1 0 1\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);

    // this chain of function is of length 3 but our limit in this test is 2, so we call foo twice
    let actual3 = compile_function(
        r#"
local function foo(a, b)
    return a + b
end

local function bar(x)
    return foo(x, 1) * foo(x, -1)
end

local function baz()
    return (bar(42))
end

return (baz())
"#,
        3,
        2,
        0,
    );
    let expected3 = "\nDUPCLOSURE R0 K0 ['foo']\nDUPCLOSURE R1 K1 ['bar']\nDUPCLOSURE R2 K2 ['baz']\nMOVE R4 R0\nLOADN R5 42\nLOADN R6 1\nCALL R4 2 1\nMOVE R5 R0\nLOADN R6 42\nLOADN R7 -1\nCALL R5 2 1\nMUL R3 R4 R5\nRETURN R3 1\n";
    assert_eq!(format!("\n{}", actual3), expected3);
}
