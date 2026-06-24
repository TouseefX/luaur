#[cfg(test)]
#[test]
fn compiler_debug_no_inline() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::DebugLuauNoInline;

    let _no_inline = ScopedFastFlag::new(&DebugLuauNoInline, true);

    let actual1 = compile_function(
        r#"@debugnoinline
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
    let expected1 = "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nCALL R1 0 1\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    let actual2 = compile_function(
        r#"@debugnoinline
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
    let expected2 = "\nDUPCLOSURE R0 K0 ['foo']\nMOVE R1 R0\nLOADB R2 1\nLOADN R3 5\nGETIMPORT R4 3 [math.random]\nCALL R4 0 -1\nCALL R1 -1 1\nRETURN R1 1\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
