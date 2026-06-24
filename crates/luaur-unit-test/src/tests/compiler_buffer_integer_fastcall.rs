#[cfg(test)]
#[test]
fn compiler_buffer_integer_fastcall() {
    use crate::functions::compile_function_0::compile_function_0;
    use crate::type_aliases::scoped_fast_flag::ScopedFastFlag;

    let luau_integer_fastcalls =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauIntegerFastcalls, true);
    let _luau_integer_buffer_fastcalls =
        ScopedFastFlag::new(&luaur_common::FFlag::LuauIntegerBufferFastcalls, true);

    let result1 = compile_function_0(
        r#"local b = buffer.create(16)
return buffer.readinteger(b, 0)
"#,
    );
    let expected1 = "\nGETIMPORT R0 2 [buffer.create]\nLOADN R1 16\nCALL R0 1 1\nFASTCALL2K 131 R0 K3 L0 [0]\nMOVE R2 R0\nLOADK R3 K3 [0]\nGETIMPORT R1 5 [buffer.readinteger]\nCALL R1 2 -1\nL0: RETURN R1 -1\n";
    assert_eq!("\n".to_string() + &result1, expected1);

    let result2 = compile_function_0(
        r#"local b, v = ...
buffer.writeinteger(b, 0, v)
"#,
    );
    let expected2 = "\nGETVARARGS R0 2\nLOADN R4 0\nFASTCALL3 132 R0 R4 R1 L0\nMOVE R3 R0\nMOVE R5 R1\nGETIMPORT R2 2 [buffer.writeinteger]\nCALL R2 3 0\nL0: RETURN R0 0\n";
    assert_eq!("\n".to_string() + &result2, expected2);
}
