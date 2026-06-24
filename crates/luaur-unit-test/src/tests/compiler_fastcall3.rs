#[cfg(test)]
#[test]
fn compiler_fastcall3() {
    use crate::functions::compile_function_0::compile_function_0;

    let actual = compile_function_0(
        "local a, b, c = ...\n\
         return math.min(a, b, c) + math.clamp(a, b, c)",
    );
    let expected = "\n\
                    GETVARARGS R0 3\n\
                    FASTCALL3 19 R0 R1 R2 L0\n\
                    MOVE R5 R0\n\
                    MOVE R6 R1\n\
                    MOVE R7 R2\n\
                    GETIMPORT R4 2 [math.min]\n\
                    CALL R4 3 1\n\
                    L0: FASTCALL3 46 R0 R1 R2 L1\n\
                    MOVE R6 R0\n\
                    MOVE R7 R1\n\
                    MOVE R8 R2\n\
                    GETIMPORT R5 4 [math.clamp]\n\
                    CALL R5 3 1\n\
                    L1: ADD R3 R4 R5\n\
                    RETURN R3 1\n";

    assert_eq!(format!("\n{}", actual), expected);
}
