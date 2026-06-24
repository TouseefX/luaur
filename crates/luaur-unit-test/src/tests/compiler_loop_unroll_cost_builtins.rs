#[cfg(test)]
#[test]
fn compiler_loop_unroll_cost_builtins() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FFlag;

    let _sfis = [
        ScopedFastInt::new(&luaur_common::FInt::LuauCompileLoopUnrollThreshold, 25),
        ScopedFastInt::new(
            &luaur_common::FInt::LuauCompileLoopUnrollThresholdMaxBoost,
            300,
        ),
    ];
    let _emit_call_fb = ScopedFastFlag::new(&FFlag::LuauEmitCallFeedback, true);

    // this loop uses builtins and is close to the cost budget so it's important that we model builtins as cheaper than regular calls
    let result = compile_function(
        r"function cipher(block, nonce)
    for i = 0,3 do
        block[i + 1] = bit32.band(bit32.rshift(nonce, i * 8), 0xff)
    end
end",
        0,
        2,
        0,
    );
    let expected = r"
FASTCALL2K 39 R1 K0 L0 [0]
MOVE R4 R1
LOADK R5 K0 [0]
GETIMPORT R3 3 [bit32.rshift]
CALL R3 2 1
L0: FASTCALL2K 29 R3 K4 L1 [255]
LOADK R4 K4 [255]
GETIMPORT R2 6 [bit32.band]
CALL R2 2 1
L1: SETTABLEN R2 R0 1
FASTCALL2K 39 R1 K7 L2 [8]
MOVE R4 R1
LOADK R5 K7 [8]
GETIMPORT R3 3 [bit32.rshift]
CALL R3 2 1
L2: FASTCALL2K 29 R3 K4 L3 [255]
LOADK R4 K4 [255]
GETIMPORT R2 6 [bit32.band]
CALL R2 2 1
L3: SETTABLEN R2 R0 2
FASTCALL2K 39 R1 K8 L4 [16]
MOVE R4 R1
LOADK R5 K8 [16]
GETIMPORT R3 3 [bit32.rshift]
CALL R3 2 1
L4: FASTCALL2K 29 R3 K4 L5 [255]
LOADK R4 K4 [255]
GETIMPORT R2 6 [bit32.band]
CALL R2 2 1
L5: SETTABLEN R2 R0 3
FASTCALL2K 39 R1 K9 L6 [24]
MOVE R4 R1
LOADK R5 K9 [24]
GETIMPORT R3 3 [bit32.rshift]
CALL R3 2 1
L6: FASTCALL2K 29 R3 K4 L7 [255]
LOADK R4 K4 [255]
GETIMPORT R2 6 [bit32.band]
CALL R2 2 1
L7: SETTABLEN R2 R0 4
RETURN R0 0
";
    assert_eq!("\n".to_string() + &result, expected);

    // note that if we break compiler's ability to reason about bit32 builtin the loop is no longer unrolled as it's too expensive
    let result = compile_function(
        r"bit32 = {}

function cipher(block, nonce)
    for i = 0,3 do
        block[i + 1] = bit32.band(bit32.rshift(nonce, i * 8), 0xff)
    end
end",
        0,
        2,
        0,
    );
    let expected = r"
LOADN R4 0
LOADN R2 3
LOADN R3 1
FORNPREP R2 L1
L0: ADDK R5 R4 K0 [1]
GETGLOBAL R6 K1 ['bit32']
GETTABLEKS R6 R6 K2 ['band']
GETGLOBAL R7 K1 ['bit32']
GETTABLEKS R7 R7 K3 ['rshift']
MOVE R8 R1
MULK R9 R4 K4 [8]
CALLFB R7 2 1 [0]
LOADN R8 255
CALLFB R6 2 1 [1]
SETTABLE R6 R0 R5
FORNLOOP R2 L0
L1: RETURN R0 0
";
    assert_eq!("\n".to_string() + &result, expected);

    // additionally, if we pass too many constants the builtin stops being cheap because of argument setup
    let result = compile_function(
        r"function cipher(block, nonce)
    for i = 0,3 do
        block[i + 1] = bit32.band(bit32.rshift(nonce, i * 8), 0xff, 0xff, 0xff, 0xff, 0xff)
    end
end",
        0,
        2,
        0,
    );
    let expected = r"
LOADN R4 0
LOADN R2 3
LOADN R3 1
FORNPREP R2 L3
L0: ADDK R5 R4 K0 [1]
MULK R9 R4 K1 [8]
FASTCALL2 39 R1 R9 L1
MOVE R8 R1
GETIMPORT R7 4 [bit32.rshift]
CALL R7 2 1
L1: LOADN R8 255
LOADN R9 255
LOADN R10 255
LOADN R11 255
LOADN R12 255
FASTCALL 29 L2
GETIMPORT R6 6 [bit32.band]
CALL R6 6 1
L2: SETTABLE R6 R0 R5
FORNLOOP R2 L0
L3: RETURN R0 0
";
    assert_eq!("\n".to_string() + &result, expected);
}
