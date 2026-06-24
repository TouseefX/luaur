#[cfg(test)]
#[test]
fn compiler_builtin_folding_multret() {
    use crate::functions::compile_function::compile_function;

    let result = compile_function(
        r#"local NoLanes: Lanes = --[[                             ]] 0b0000000000000000000000000000000
local OffscreenLane: Lane = --[[                        ]] 0b1000000000000000000000000000000

local function getLanesToRetrySynchronouslyOnError(root: FiberRoot): Lanes
    local everythingButOffscreen = bit32.band(root.pendingLanes, bit32.bnot(OffscreenLane))
    if everythingButOffscreen ~= NoLanes then
        return everythingButOffscreen
    end
    if bit32.band(everythingButOffscreen, OffscreenLane) ~= 0 then
        return OffscreenLane
    end
    return NoLanes
end
"#,
        0,
        2,
        0,
    );

    let expected = "\nGETTABLEKS R2 R0 K0 ['pendingLanes']\nFASTCALL2K 29 R2 K1 L0 [3221225471]\nLOADK R3 K1 [3221225471]\nGETIMPORT R1 4 [bit32.band]\nCALL R1 2 1\nL0: JUMPXEQKN R1 K5 L1 [0]\nRETURN R1 1\nL1: FASTCALL2K 29 R1 K6 L2 [1073741824]\nMOVE R3 R1\nLOADK R4 K6 [1073741824]\nGETIMPORT R2 4 [bit32.band]\nCALL R2 2 1\nL2: JUMPXEQKN R2 K5 L3 [0]\nLOADK R2 K6 [1073741824]\nRETURN R2 1\nL3: LOADN R2 0\nRETURN R2 1\n";
    assert_eq!("\n".to_string() + &result, expected);

    let result = compile_function(
        r#"return math.abs(-42)
"#,
        0,
        2,
        0,
    );

    let expected = "\nLOADN R0 42\nRETURN R0 1\n";
    assert_eq!("\n".to_string() + &result, expected);
}
