#[cfg(test)]
#[test]
fn compiler_elide_jump_after_if() {
    use crate::functions::compile_function_0::compile_function_0;

    // break refers to outer loop => we can elide unconditional branches
    let actual1 = compile_function_0(
        "local foo, bar = ...\n\
         repeat\n\
             if foo then break\n\
             elseif bar then break\n\
             end\n\
             print(1234)\n\
         until foo == bar\n",
    );
    let expected1 = "\n\
                     GETVARARGS R0 2\n\
                     L0: JUMPIFNOT R0 L1\n\
                     RETURN R0 0\n\
                     L1: JUMPIF R1 L2\n\
                     GETIMPORT R2 1 [print]\n\
                     LOADN R3 1234\n\
                     CALL R2 1 0\n\
                     JUMPIFEQ R0 R1 L2\n\
                     JUMPBACK L0\n\
                     L2: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual1), expected1);

    // break refers to inner loop => branches remain
    let actual2 = compile_function_0(
        "local foo, bar = ...\n\
         repeat\n\
             if foo then while true do break end\n\
             elseif bar then while true do break end\n\
             end\n\
             print(1234)\n\
         until foo == bar\n",
    );
    let expected2 = "\n\
                     GETVARARGS R0 2\n\
                     L0: JUMPIFNOT R0 L1\n\
                     JUMP L2\n\
                     JUMPBACK L2\n\
                     JUMP L2\n\
                     L1: JUMPIFNOT R1 L2\n\
                     JUMP L2\n\
                     JUMPBACK L2\n\
                     L2: GETIMPORT R2 1 [print]\n\
                     LOADN R3 1234\n\
                     CALL R2 1 0\n\
                     JUMPIFEQ R0 R1 L3\n\
                     JUMPBACK L0\n\
                     L3: RETURN R0 0\n";
    assert_eq!(format!("\n{}", actual2), expected2);
}
