#[cfg(test)]
#[test]
fn compiler_inline_prohibited_recursion() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let _emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    let result1 = compile_function(
        r#"local function fact(n)
    return if n <= 1 then 1 else fact(n-1)*n
end

return fact
"#,
        0,
        2,
        0,
    );
    let expected1 = r#"
LOADN R2 1
JUMPIFNOTLE R0 R2 L0
LOADN R1 1
RETURN R1 1
L0: GETUPVAL R2 0
SUBK R3 R0 K0 [1]
CALLFB R2 1 1 [0]
MUL R1 R2 R0
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", result1), expected1);

    let result2 = compile_function(
        r#"local function fact(n)
    return if n <= 1 then 1 else fact(n-1)*n
end

local function factsafe(n)
    assert(n >= 1)
    return fact(n)
end

return factsafe
"#,
        1,
        2,
        0,
    );
    let expected2 = r#"
LOADN R3 1
JUMPIFLE R3 R0 L0
LOADB R2 0 +1
L0: LOADB R2 1
L1: FASTCALL1 1 R2 L2
GETIMPORT R1 1 [assert]
CALL R1 1 0
L2: LOADN R2 1
JUMPIFNOTLE R0 R2 L3
LOADN R1 1
RETURN R1 1
L3: GETUPVAL R2 0
SUBK R3 R0 K2 [1]
CALLFB R2 1 1 [0]
MUL R1 R2 R0
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", result2), expected2);
}
