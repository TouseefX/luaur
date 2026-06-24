#[cfg(test)]
#[test]
fn compiler_loop_unroll_cost() {
    use crate::functions::compile_function::compile_function;
    use crate::type_aliases::scoped_fast_int::ScopedFastInt;
    use luaur_common::FFlag;
    use luaur_common::FInt;

    let _sfis = [
        ScopedFastInt::new(&FInt::LuauCompileLoopUnrollThreshold, 25),
        ScopedFastInt::new(&FInt::LuauCompileLoopUnrollThresholdMaxBoost, 300),
    ];

    let actual = "\n".to_string()
        + &compile_function(
            r#"local t = {}
for i=1,10 do
    t[i] = i
end
return t
"#,
            0,
            2,
            0,
        );
    let expected = "\n\
NEWTABLE R0 0 10
LOADN R1 1
SETTABLEN R1 R0 1
LOADN R1 2
SETTABLEN R1 R0 2
LOADN R1 3
SETTABLEN R1 R0 3
LOADN R1 4
SETTABLEN R1 R0 4
LOADN R1 5
SETTABLEN R1 R0 5
LOADN R1 6
SETTABLEN R1 R0 6
LOADN R1 7
SETTABLEN R1 R0 7
LOADN R1 8
SETTABLEN R1 R0 8
LOADN R1 9
SETTABLEN R1 R0 9
LOADN R1 10
SETTABLEN R1 R0 10
RETURN R0 1
";
    assert_eq!(actual, expected);

    let actual2 = "\n".to_string()
        + &compile_function(
            r#"
local t = {}
for i=1,100 do
    t[i] = i
end
return t
"#,
            0,
            2,
            0,
        );
    let expected2 = "\n\
NEWTABLE R0 0 0
LOADN R3 1
LOADN R1 100
LOADN R2 1
FORNPREP R1 L1
L0: SETTABLE R3 R0 R3
FORNLOOP R1 L0
L1: RETURN R0 1
";
    assert_eq!(actual2, expected2);

    let actual3 = "\n".to_string()
        + &compile_function(
            r#"local t = {}
for i=1,25 do
    t[i] = i * i * i
end
return t
"#,
            0,
            2,
            0,
        );
    let expected3 = "\n\
NEWTABLE R0 0 0
LOADN R1 1
SETTABLEN R1 R0 1
LOADN R1 8
SETTABLEN R1 R0 2
LOADN R1 27
SETTABLEN R1 R0 3
LOADN R1 64
SETTABLEN R1 R0 4
LOADN R1 125
SETTABLEN R1 R0 5
LOADN R1 216
SETTABLEN R1 R0 6
LOADN R1 343
SETTABLEN R1 R0 7
LOADN R1 512
SETTABLEN R1 R0 8
LOADN R1 729
SETTABLEN R1 R0 9
LOADN R1 1000
SETTABLEN R1 R0 10
LOADN R1 1331
SETTABLEN R1 R0 11
LOADN R1 1728
SETTABLEN R1 R0 12
LOADN R1 2197
SETTABLEN R1 R0 13
LOADN R1 2744
SETTABLEN R1 R0 14
LOADN R1 3375
SETTABLEN R1 R0 15
LOADN R1 4096
SETTABLEN R1 R0 16
LOADN R1 4913
SETTABLEN R1 R0 17
LOADN R1 5832
SETTABLEN R1 R0 18
LOADN R1 6859
SETTABLEN R1 R0 19
LOADN R1 8000
SETTABLEN R1 R0 20
LOADN R1 9261
SETTABLEN R1 R0 21
LOADN R1 10648
SETTABLEN R1 R0 22
LOADN R1 12167
SETTABLEN R1 R0 23
LOADN R1 13824
SETTABLEN R1 R0 24
LOADN R1 15625
SETTABLEN R1 R0 25
RETURN R0 1
";
    assert_eq!(actual3, expected3);

    let actual4 = "\n".to_string()
        + &compile_function(
            r#"local t = {}
for i=1,10 do
    t[i] = math.abs(math.sin(i))
end
return t
"#,
            0,
            2,
            0,
        );
    let expected4 = "\n\
NEWTABLE R0 0 10
LOADN R3 1
LOADN R1 10
LOADN R2 1
FORNPREP R1 L3
L0: FASTCALL1 24 R3 L1
MOVE R6 R3
GETIMPORT R5 2 [math.sin]
CALL R5 1 1
L1: FASTCALL1 2 R5 L2
GETIMPORT R4 4 [math.abs]
CALL R4 1 1
L2: SETTABLE R4 R0 R3
FORNLOOP R1 L0
L3: RETURN R0 1
";
    assert_eq!(actual4, expected4);
}
