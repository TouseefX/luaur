#[cfg(test)]
#[test]
fn compiler_jump_fold() {
    use crate::functions::compile_function::compile_function;
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let emit_call_fb = ScopedFastFlag::new(&luaur_common::FFlag::LuauEmitCallFeedback, true);

    // jump-to-return folding to return
    let actual = compile_function_0("return a and 1 or 0");
    let expected = "\nGETIMPORT R1 1 [a]\nJUMPIFNOT R1 L0\nLOADN R0 1\nRETURN R0 1\nL0: LOADN R0 0\nRETURN R0 1\n";
    assert_eq!(format!("\n{}", actual), expected);

    // conditional jump in the inner if() folding to jump out of the expression (JUMPIFNOT+5 skips over all jumps, JUMP+1 skips over JUMP+0)
    let actual = compile_function_0("if a then if b then b() else end else end d()");
    let expected = "\nGETIMPORT R0 1 [a]\nJUMPIFNOT R0 L0\nGETIMPORT R0 3 [b]\nJUMPIFNOT R0 L0\nGETIMPORT R0 3 [b]\nCALL R0 0 0\nJUMP L0\nJUMP L0\nL0: GETIMPORT R0 5 [d]\nCALL R0 0 0\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // same as example before but the unconditional jumps are folded with RETURN
    let actual = compile_function_0("if a then if b then b() else end else end");
    let expected = "\nGETIMPORT R0 1 [a]\nJUMPIFNOT R0 L0\nGETIMPORT R0 3 [b]\nJUMPIFNOT R0 L0\nGETIMPORT R0 3 [b]\nCALL R0 0 0\nRETURN R0 0\nRETURN R0 0\nL0: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    // in this example, we do *not* have a JUMP after RETURN in the if branch
    // this is important since, even though this jump is never reached, jump folding needs to be able to analyze it
    let actual = compile_function(
        r#"local function getPerlin(x, y, z, seed, scale, raw)
local seed = seed or 0
local scale = scale or 1
if not raw then
return math.noise(x / scale + (seed * 17) + masterSeed, y / scale - masterSeed, z / scale - seed*seed)*.5 + .5 --accounts for bleeding from interpolated line
else
return math.noise(x / scale + (seed * 17) + masterSeed, y / scale - masterSeed, z / scale - seed*seed)
end
end
"#,
        0,
        1,
        0,
    );
    let expected = "\nORK R6 R3 K0 [0]\nORK R7 R4 K1 [1]\nJUMPIF R5 L0\nGETIMPORT R10 5 [math.noise]\nDIV R13 R0 R7\nMULK R14 R6 K6 [17]\nADD R12 R13 R14\nGETIMPORT R13 8 [masterSeed]\nADD R11 R12 R13\nDIV R13 R1 R7\nGETIMPORT R14 8 [masterSeed]\nSUB R12 R13 R14\nDIV R14 R2 R7\nMUL R15 R6 R6\nSUB R13 R14 R15\nCALLFB R10 3 1 [0]\nMULK R9 R10 K2 [0.5]\nADDK R8 R9 K2 [0.5]\nRETURN R8 1\nL0: GETIMPORT R8 5 [math.noise]\nDIV R11 R0 R7\nMULK R12 R6 K6 [17]\nADD R10 R11 R12\nGETIMPORT R11 8 [masterSeed]\nADD R9 R10 R11\nDIV R11 R1 R7\nGETIMPORT R12 8 [masterSeed]\nSUB R10 R11 R12\nDIV R12 R2 R7\nMUL R13 R6 R6\nSUB R11 R12 R13\nCALL R8 3 -1\nRETURN R8 -1\n";
    assert_eq!(format!("\n{}", actual), expected);

    drop(emit_call_fb);
}
