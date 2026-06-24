#[cfg(test)]
#[test]
fn compiler_return_consecutive() {
    use crate::functions::compile_function_0::compile_function_0;

    // we can return a single local directly
    assert_eq!(
        compile_function_0("local x = ...\nreturn x"),
        "GETVARARGS R0 1\nRETURN R0 1\n"
    );

    // or multiple, when they are allocated in consecutive registers
    assert_eq!(
        compile_function_0("local x, y = ...\nreturn x, y"),
        "GETVARARGS R0 2\nRETURN R0 2\n"
    );

    // but not if it's an expression
    assert_eq!(
        compile_function_0("local x, y = ...\nreturn x, y + 1"),
        "GETVARARGS R0 2\nMOVE R2 R0\nADDK R3 R1 K0 [1]\nRETURN R2 2\n"
    );

    // or a local with wrong register number
    assert_eq!(
        compile_function_0("local x, y = ...\nreturn y, x"),
        "GETVARARGS R0 2\nMOVE R2 R1\nMOVE R3 R0\nRETURN R2 2\n"
    );

    // also double check the optimization doesn't trip on no-argument return (these are rare)
    assert_eq!(compile_function_0("return"), "RETURN R0 0\n");

    // this optimization also works in presence of group / type casts
    assert_eq!(
        compile_function_0("local x, y = ...\nreturn (x), y :: number"),
        "GETVARARGS R0 2\nRETURN R0 2\n"
    );
}
