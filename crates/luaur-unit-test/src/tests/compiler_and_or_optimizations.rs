#[cfg(test)]
#[test]
fn compiler_and_or_optimizations() {
    use crate::functions::compile_function::compile_function;

    // the OR/ORK optimization triggers for cutoff since lhs is simple
    let actual = compile_function(
        r#"local function advancedRidgedFilter(value, cutoff)
    local cutoff = cutoff or .5
    value = value - cutoff
    return 1 - (value < 0 and -value or value) * 1 / (1 - cutoff)
end
"#,
        0,
        1,
        0,
    );
    let expected = r#"
ORK R2 R1 K0 [0.5]
SUB R0 R0 R2
LOADN R7 0
JUMPIFNOTLT R0 R7 L0
MINUS R6 R0
JUMPIF R6 L1
L0: MOVE R6 R0
L1: MULK R5 R6 K1 [1]
SUBRK R6 K1 [1] R2
DIV R4 R5 R6
SUBRK R3 K1 [1] R4
RETURN R3 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // sometimes we need to compute a boolean; this uses LOADB with an offset
    let actual = compile_function(
        r#"function thinSurface(surfaceGradient, surfaceThickness)
    return surfaceGradient > .5 - surfaceThickness*.4 and surfaceGradient < .5 + surfaceThickness*.4
end
"#,
        0,
        1,
        0,
    );
    let expected = r#"
LOADB R2 0
MULK R4 R1 K1 [0.40000000000000002]
SUBRK R3 K0 [0.5] R4
JUMPIFNOTLT R3 R0 L1
LOADK R4 K0 [0.5]
MULK R5 R1 K1 [0.40000000000000002]
ADD R3 R4 R5
JUMPIFLT R0 R3 L0
LOADB R2 0 +1
L0: LOADB R2 1
L1: RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // sometimes we need to compute a boolean; this uses LOADB with an offset for the last op, note that first op is compiled better
    let actual = compile_function(
        r#"function thickSurface(surfaceGradient, surfaceThickness)
    return surfaceGradient < .5 - surfaceThickness*.4 or surfaceGradient > .5 + surfaceThickness*.4
end
"#,
        0,
        1,
        0,
    );
    let expected = r#"
LOADB R2 1
MULK R4 R1 K1 [0.40000000000000002]
SUBRK R3 K0 [0.5] R4
JUMPIFLT R0 R3 L1
LOADK R4 K0 [0.5]
MULK R5 R1 K1 [0.40000000000000002]
ADD R3 R4 R5
JUMPIFLT R3 R0 L0
LOADB R2 0 +1
L0: LOADB R2 1
L1: RETURN R2 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // trivial ternary if with constants
    let actual = compile_function(
        r#"function testSurface(surface)
    return surface and 1 or 0
end
"#,
        0,
        1,
        0,
    );
    let expected = r#"
JUMPIFNOT R0 L0
LOADN R1 1
RETURN R1 1
L0: LOADN R1 0
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", actual), expected);

    // canonical saturate
    let actual = compile_function(
        r#"function saturate(x)
    return x < 0 and 0 or x > 1 and 1 or x
end
"#,
        0,
        1,
        0,
    );
    let expected = r#"
LOADN R2 0
JUMPIFNOTLT R0 R2 L0
LOADN R1 0
RETURN R1 1
L0: LOADN R2 1
JUMPIFNOTLT R2 R0 L1
LOADN R1 1
RETURN R1 1
L1: MOVE R1 R0
RETURN R1 1
"#;
    assert_eq!(format!("\n{}", actual), expected);
}
