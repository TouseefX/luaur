#[cfg(test)]
#[test]
fn compiler_for_bytecode_builtin() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;
    use luaur_common::FFlag::LuauEmitCallFeedback;

    let _emit_call_fb = ScopedFastFlag::new(&LuauEmitCallFeedback, true);

    let actual = compile_function_0("for k,v in ipairs({}) do end");
    let expected = "\nGETIMPORT R0 1 [ipairs]\nNEWTABLE R1 0 0\nCALL R0 1 3\nFORGPREP_INEXT R0 L0\nL0: FORGLOOP R0 L0 2 [inext]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local ip = ipairs for k,v in ip({}) do end");
    let expected = "\nGETIMPORT R0 1 [ipairs]\nMOVE R1 R0\nNEWTABLE R2 0 0\nCALL R1 1 3\nFORGPREP_INEXT R1 L0\nL0: FORGLOOP R1 L0 2 [inext]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual =
        compile_function_0("local ip = ipairs function foo() for k,v in ip({}) do end end");
    let expected = "\nGETUPVAL R0 0\nNEWTABLE R1 0 0\nCALLFB R0 1 3 [0]\nFORGPREP_INEXT R0 L0\nL0: FORGLOOP R0 L0 2 [inext]\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("local ip = ipairs ip = pairs for k,v in ip({}) do end");
    let expected = "\nGETIMPORT R0 1 [ipairs]\nGETIMPORT R0 3 [pairs]\nMOVE R1 R0\nNEWTABLE R2 0 0\nCALL R1 1 3\nFORGPREP R1 L0\nL0: FORGLOOP R1 L0 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("ipairs = pairs for k,v in ipairs({}) do end");
    let expected = "\nGETIMPORT R0 1 [pairs]\nSETGLOBAL R0 K2 ['ipairs']\nGETGLOBAL R0 K2 ['ipairs']\nNEWTABLE R1 0 0\nCALL R0 1 3\nFORGPREP R0 L0\nL0: FORGLOOP R0 L0 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);

    let actual = compile_function_0("for k,v in unknown({}) do end");
    let expected = "\nGETIMPORT R0 1 [unknown]\nNEWTABLE R1 0 0\nCALL R0 1 3\nFORGPREP R0 L0\nL0: FORGLOOP R0 L0 2\nRETURN R0 0\n";
    assert_eq!(format!("\n{}", actual), expected);
}
