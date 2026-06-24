#[cfg(test)]
#[test]
fn compiler_loop_unroll_control_flow() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FInt;

    let _sfis = [
        ScopedFastInt::new(&FInt::LuauCompileLoopUnrollThreshold, 50),
        ScopedFastInt::new(&FInt::LuauCompileLoopUnrollThresholdMaxBoost, 300),
    ];

    // break jumps to the end
    let actual = compile_function(
        r#"
for i=1,3 do
    if math.random() < 0.5 then
        break
    end
end
"#,
        0,
        2,
        0,
    );
    let expected = r#"
GETIMPORT R0 2 [math.random]
CALL R0 0 1
LOADK R1 K3 [0.5]
JUMPIFLT R0 R1 L0
GETIMPORT R0 2 [math.random]
CALL R0 0 1
LOADK R1 K3 [0.5]
JUMPIFLT R0 R1 L0
GETIMPORT R0 2 [math.random]
CALL R0 0 1
LOADK R1 K3 [0.5]
JUMPIFLT R0 R1 L0
L0: RETURN R0 0
"#;
    assert_eq!(actual.trim(), expected.trim());

    // continue jumps to the next iteration
    let actual = compile_function(
        r#"
for i=1,3 do
    if math.random() < 0.5 then
        continue
    end
    print(i)
end
"#,
        0,
        2,
        0,
    );
    let expected = r#"
GETIMPORT R0 2 [math.random]
CALL R0 0 1
LOADK R1 K3 [0.5]
JUMPIFLT R0 R1 L0
GETIMPORT R0 5 [print]
LOADN R1 1
CALL R0 1 0
L0: GETIMPORT R0 2 [math.random]
CALL R0 0 1
LOADK R1 K3 [0.5]
JUMPIFLT R0 R1 L1
GETIMPORT R0 5 [print]
LOADN R1 2
CALL R0 1 0
L1: GETIMPORT R0 2 [math.random]
CALL R0 0 1
LOADK R1 K3 [0.5]
JUMPIFLT R0 R1 L2
GETIMPORT R0 5 [print]
LOADN R1 3
CALL R0 1 0
L2: RETURN R0 0
"#;
    assert_eq!(actual.trim(), expected.trim());

    // continue needs to properly close upvalues
    let actual = compile_function(
        r#"
for i=1,1 do
    local j = global(i)
    print(function() return j end)
    if math.random() < 0.5 then
        continue
    end
    j += 1
end
"#,
        1,
        2,
        0,
    );
    let expected = r#"
GETIMPORT R0 1 [global]
LOADN R1 1
CALL R0 1 1
GETIMPORT R1 3 [print]
NEWCLOSURE R2 P0
CAPTURE REF R0
CALL R1 1 0
GETIMPORT R1 6 [math.random]
CALL R1 0 1
LOADK R2 K7 [0.5]
JUMPIFNOTLT R1 R2 L0
CLOSEUPVALS R0
RETURN R0 0
L0: ADDK R0 R0 K8 [1]
CLOSEUPVALS R0
RETURN R0 0
"#;
    assert_eq!(actual.trim(), expected.trim());

    // this weird contraption just disappears
    let actual = compile_function(
        r#"
for i=1,1 do
    for j=1,1 do
        if i == 1 then
            continue
        else
            break
        end
    end
end
"#,
        0,
        2,
        0,
    );
    let expected = r#"
RETURN R0 0
RETURN R0 0
"#;
    assert_eq!(actual.trim(), expected.trim());
}
